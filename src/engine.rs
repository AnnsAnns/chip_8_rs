/*
* Copyright 2021 tomGER, git@tomger.eu
*
* Licensed under the EUPL, Version 1.2 or – as soon they will be approved by the European Commission - subsequent versions of the EUPL (the "Licence");
* You may not use this work except in compliance with theLicence.
*
* You may obtain a copy of the Licence at: https://joinup.ec.europa.eu/software/page/eupl
*
* Unless required by applicable law or agreed to in writing, software distributed under the Licence is distributed on an "AS IS" basis,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the Licence for the specific language governing permissions and limitations under the Licence.
*/

use std::fs;
use rand;
use rand::Rng;

use crate::font::FONT_SET;
use crate::program_counter::ProgramCounter;
use crate::HEIGHT;
use crate::WIDTH;

struct CPUCycle {
    opcode: u16,
    nnn: u16,
    nn: u8,
    n: u8,
    x: usize,
    y: usize,
}

impl CPUCycle {
    fn get_kk(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }
}

pub struct Engine {
    // The Main Engine of the Emulator
    // Mostly taken from my older emulator: https://github.com/tumGER/CHIP-8/blob/master/main.go
    memory: [u8; 4096],
    //0x0 - 0x1FF Chip8 Interpreter
	//0x050-0x0A0 Fonts
	//0x200-0xFFF Program and RAM

    opcode: u16,
    v: [u8; 16], // CPU register
    i: u16, // Index register
    pc: u16, // Program counter
    gfx: [[u8; WIDTH]; HEIGHT], // Screen

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stackpointer: u8,

    key: [bool; 16], // Input
    pub pressed_key: u8,
    pub waiting_for_input: bool,
    pub waiting_for_draw: bool,
    pub draw_flag: bool, // Disable actually drawing to the screen
}

impl Engine {
    pub fn new(draw: bool) -> Self {
        // Init new structure
        Self {   
            memory: [0; 4096],
            opcode: 0,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; WIDTH]; HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stackpointer: 0,
            key: [false; 16],
            pressed_key: 2,
            waiting_for_input: false,
            waiting_for_draw: false,
            draw_flag: draw,
        }
    }

    pub fn clear(&mut self) {
        // Clear variables
        self.pc = 0x200;
        self.opcode = 0;
        self.i = 0;
        self.stackpointer = 0;

        // @TODO: Clear more stuff
    }

    pub fn read_font(&mut self) {
        for i in 0..FONT_SET.len() {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn read_game(&mut self, file_name: &str) {
        let buffer = fs::read(file_name).unwrap();
        let mut n: usize = 512;

        for byte in buffer.into_iter() {
            self.memory[n] = byte;

            n += 1; // Is there a better solution?
        }
    }

    pub fn cycle(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16; // Get next opcode
        let cycle = CPUCycle {
            opcode: self.opcode,
            nnn: self.opcode & 0x0FFF,
            nn: (self.opcode & 0x0FF) as u8,
            n: (self.opcode & 0x00F) as u8,
            x: (self.opcode >> 8 & 0x000F) as usize,
            y: (self.opcode >> 4 & 0x000F) as usize,
        };
        println!("{:X}", cycle.opcode); // Debug Info

        // Decode opcode, pc += 2 -> next cycle, pc += 4 -> skip cycle
        let next_pc = match (self.opcode & 0xF000) >> 12 {
            0x0 => {
                match cycle.nn {
                    0xE0 => { // 0x00E0: Clears the screen
                        for x in 0..WIDTH {
                            for y in 0..HEIGHT {
                                self.gfx[x][y] = 0
                            }
                        }
                        ProgramCounter::Next
                    }
                    0xEE => { // 0x00EE: Returns from subroutine
                        self.stackpointer -= 1; // Go back by one
                        ProgramCounter::Jump(self.stack[self.stackpointer as usize] as u16)
                    }
                    _ => panic!("Unknown opcode: {:X}", self.opcode)
                }
            }
            0x1 => { // 1NNN: Jump to location nnn.
                ProgramCounter::Jump(cycle.nnn)
            }
            0x2 => { // 2NNN: Calls subroutine at address NNN
                self.stack[self.stackpointer as usize] = self.pc + 2;
                self.stackpointer += 1;

                ProgramCounter::Jump(cycle.nnn)
            }
            0x3 => { // 3XKK Skip next instruction if Vx = kk.
                ProgramCounter::skip_when(self.v[cycle.x] == cycle.get_kk())
            }
            0x4 => { // 4XKK Skip next instruction if NOT Vx = kk.
                ProgramCounter::skip_when(self.v[cycle.x] != cycle.get_kk())
            }
            0x5 => { // 5xy0: Skip next instruction if Vx = Vy.
                ProgramCounter::skip_when(self.v[cycle.x] == self.v[cycle.y])
            }
            0x6 => { // 6xkk: Set Vx = kk
                self.v[cycle.x] = cycle.get_kk();
                ProgramCounter::Next
            }
            0x7 => { // 7xkk: Vx = Vx + kk.
                let vx = self.v[cycle.x] as u16;
                let val = cycle.get_kk() as u16;
                let result = vx + val;
                self.v[cycle.x] = result as u8;
                ProgramCounter::Next
            }
            0x8 => {
                match cycle.n {
                    0x0 => { // 8XY0: Set Vx = Vy
                        self.v[cycle.x] = self.v[cycle.y];
                        ProgramCounter::Next
                    }
                    0x1 => { // 8XY1: set Vx = Vx OR Vy.
                        self.v[cycle.x as usize] |= self.v[cycle.y];
                        ProgramCounter::Next
                    }
                    0x2 => { // 8XY2: Set Vx = Vx AND Vy. 
                        self.v[cycle.x] &= self.v[cycle.y];
                        ProgramCounter::Next
                    }
                    0x3 => { // 8XY3: Set Vx = Vx XOR Vy.
                        self.v[cycle.x] ^= self.v[cycle.y];
                        ProgramCounter::Next
                    }
                    0x4 => { // Add the value of register VY to register VX
                        // Set VF to 01 if a carry occurs
                        // Set VF to 00 if a carry does not occur
                        let add: u16 = self.v[cycle.x] as u16 + self.v[cycle.y] as u16;
                        self.v[cycle.x] = add as u8;
                        if add > 0xFF { // carry occurs
                            self.v[0xF] = 1
                        } else {
                            self.v[0xF] = 0 // Might be wrong ... ?
                        }

                        ProgramCounter::Next
                    }
                    0x5 => { // 8XY5: Set Vx = Vx - Vy, set VF = NOT borrow.
                        // Subtract the value of register VY from register VX
                        // Set VF to 00 if a borrow occurs
                        // Set VF to 01 if a borrow does not occur
                        self.v[0xF] = if self.v[cycle.x] > self.v[cycle.y] {1} else {0};
                        self.v[cycle.x] = self.v[cycle.x].wrapping_sub(self.v[cycle.y]);

                        ProgramCounter::Next
                    }
                    0x6 => { // 8XY6: Set Vx = Vx SHR 1.
                        // Store the value of register VY shifted right one bit in register VX
                        // Set register VF to the least significant bit prior to the shift
                        self.v[0xF] = self.v[cycle.x] & 1;
                        self.v[cycle.x] >>= 1;

                        ProgramCounter::Next
                    }
                    0x7 => { // Set register VX to the value of VY minus VX
                        // Set VF to 00 if a borrow occurs
                        //Set VF to 01 if a borrow does not occur
                        let sub: i8 = self.v[cycle.y] as i8 - self.v[cycle.x] as i8; // has to be signed since it could be negative
                        
                        if sub < 0 {
                            self.v[0xF] = 1
                        } else {
                            self.v[0xF] = 0 // I think
                        }

                        self.v[cycle.x] = sub as u8;

                        ProgramCounter::Next
                    }
                    0xE => { // Store the value of register VY shifted left one bit in register VX
                        // Set register VF to the most significant bit prior to the shift
                        self.v[0xF] = (self.v[cycle.x] & 0x80) >> 7;
                        self.v[cycle.x] <<= 1;

                        ProgramCounter::Next
                    }
                    _ => panic!("Unknown opcode: {:X}", self.opcode)
                }
            }
            0x9 => { // 9XY0: Skip next instruction if Vx != Vy.
                ProgramCounter::skip_when(self.v[cycle.x] != self.v[cycle.y])
            }
            0xA => { // ANNN: Sets I to the address NNN
                self.i = cycle.nnn;

                ProgramCounter::Next
            }
            0xB => { // BNNN: Jump to location nnn + V0
                ProgramCounter::Jump(cycle.nnn + self.v[0] as u16)
            }
            0xC => { // Cxkk: Set Vx = random byte AND kk.
                let mut random_gen = rand::thread_rng();
                self.v[cycle.x] = random_gen.gen::<u8>() & cycle.get_kk();
                // ty to https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs#L327

                ProgramCounter::Next
            }
            0xD => { // Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
                // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
                self.v[0xF] = 0;

                for byte in 0..cycle.n {
                    let y = (self.v[cycle.y] + byte) as usize % HEIGHT;
                    for bit in 0..8 {
                        let x = (self.v[cycle.x] + bit) as usize % WIDTH;
                        let colour = (self.memory[(self.i + byte as u16) as usize] >> (7 - bit)) & 1;
                        self.v[0xF] |= colour & self.gfx[y][x];
                        self.gfx[y][x] ^= colour;
                    }
                }

                self.waiting_for_draw = true;
                ProgramCounter::Next
            }
            0xE => {
                match cycle.nn {
                    0xA1 => { // ExA1: Skip next instruction if key with the value of Vx is not pressed.
                        ProgramCounter::skip_when(!self.key[self.v[cycle.x] as usize])
                    }
                    0x9E => { // Ex9E: Skip next instruction if key with the value of Vx is pressed
                        ProgramCounter::skip_when(self.key[self.v[cycle.x] as usize])
                    }
                    _ => panic!("Unknown opcode: {:X}", self.opcode)
                }
            }
            0xF => {
                match cycle.nn {
                    0x07 => { // Store the current value of the delay timer in register VX
                        self.v[cycle.x] = self.delay_timer;

                        ProgramCounter::Next
                    }
                    0x0A => { // FX0A: Wait for a key press, store the value of the key in Vx.
                        self.waiting_for_input = true;
                        self.v[cycle.x] = self.pressed_key;

                        ProgramCounter::Next
                    }
                    0x15 => { // FX15: Set delay timer = Vx.
                        self.delay_timer = self.v[cycle.x];

                        ProgramCounter::Next
                    }
                    0x18 => { // FX18: Set sound timer = Vx.
                        self.sound_timer = self.v[cycle.x];

                        ProgramCounter::Next
                    }
                    0x1E => { // FX1E: Set I = I + Vx.
                        self.i += self.v[cycle.x] as u16;
                        if self.i > 0xF {
                            self.v[0xF] = 1
                        } else {
                            self.v[0xF] = 0
                        }

                        ProgramCounter::Next
                    }
                    0x29 => {// FX29: Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
                        self.i = self.v[cycle.x] as u16 * 5; // A sprite is 5 lines big

                        ProgramCounter::Next
                    }
                    0x33 => { // FX33: Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I+1, and I+2
                        // ty to https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs#L408
                        self.memory[self.i as usize] = self.v[cycle.x] / 100;
                        self.memory[self.i as usize + 1] = (self.v[cycle.x] % 100) / 10;
                        self.memory[self.i as usize + 2] = self.v[cycle.x] % 10;

                        ProgramCounter::Next
                    }
                    0x55 => { // FX55: Store registers V0 through Vx in memory starting at location I.
                        // I is set to I + X + 1 after operation
                        for byte in 0..cycle.x + 1 {
                            self.memory[(self.i + byte as u16) as usize] = self.v[byte as usize];
                        }

                        self.i += cycle.x as u16 + 1; // Might be wrong

                        ProgramCounter::Next
                    }
                    0x65 => { // FX65: Read registers V0 through Vx from memory starting at location I.
                        for byte in 0..cycle.x + 1 {
                            self.v[byte] = self.memory[(self.i + byte as u16) as usize];
                        } 

                        ProgramCounter::Next
                    }
                    _ => panic!("Unknown opcode: {:X}", self.opcode)
                }
            }
            _ => panic!("Unknown opcode: {:X}", self.opcode)
        };

        self.pc = next_pc.resolve(self.pc);
        println!("Executed opcode {:X} correctly!", self.opcode)
    }
}