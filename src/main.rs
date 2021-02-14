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

struct Engine {
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
    gfx: [u8; 64 * 32], // Screen

    delay_timer: char,
    sound_timer: char,

    stack: [u8; 16],
    stackpointer: u8,

    key: [char; 16], // Input

    draw_flag: bool, // Disable actually drawing to the screen
}

enum ProgramCounter {
    Unknown,
    Next,
    Skip,
    Jump(u16)
}

struct CPUCycle {
    opcode: u16,
    nnn: u16,
    nn: u8,
    n: u8,
    x: u8,
    y: u8,
}

impl ProgramCounter {
    fn skip_when(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    fn resolve(&self) -> u16 {
        match self {
            ProgramCounter::Next => 2,
            ProgramCounter::Skip => 4,
            ProgramCounter::Jump(line) => {
                *line as u16
            }
            ProgramCounter::Unknown => panic!("Something went wrong and it appears like the ProgramCounter was never changed!")
        }
    }
}

impl CPUCycle {
    fn get_kk(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }
}

impl Engine {
    fn init(&mut self) {
        // Clear variables
        self.pc = 0x200;
        self.opcode = 0;
        self.i = 0;
        self.stackpointer = 0;

        // @TODO: Clear more stuff
    }

    fn read_font(&mut self) {
        // @TODO: Read font
    }

    fn read_game(&mut self, file_name: &str) {
        let buffer = fs::read(file_name).unwrap();
        let mut n: usize = 512;

        for byte in buffer.into_iter() {
            self.memory[n] = byte;
            println!("{}", self.memory[n]); // Debug

            n += 1; // Is there a better solution?
        }
    }

    fn cycle(&mut self) {
        self.opcode = self.memory[self.pc as usize] as u16 >> 8 | self.memory[self.pc as usize + 1] as u16; // Get next opcode
        let mut cycle = CPUCycle {
            opcode: self.opcode,
            nnn: self.opcode & 0x0FFF,
            nn: (self.opcode & 0x0FF) as u8,
            n: (self.opcode & 0x00F) as u8,
            x: (self.opcode >> 8 & 0x000F) as u8,
            y: (self.opcode >> 4 & 0x000F) as u8,
        };
        println!("{}", cycle.opcode); // Debug Info
        // @TODO: Everything lol

        // Decode opcode, pc += 2 -> next cycle, pc += 4 -> skip cycle
        let next_pc = match (self.opcode & 0xF000) >> 12 {
            0x0 => {
                match cycle.nn {
                    0xE0 => { // 0x00E0: Clears the screen
                        for byte in self.gfx.iter_mut() {
                            *byte = 0;
                        }

                        ProgramCounter::Next
                    }
                    0xEE => { // 0x00EE: Returns from subroutine
                        // @TODO: Implement
                        ProgramCounter::Unknown
                    }
                    _ => panic!("Unknown opcode: {}", self.opcode)
                }
            }
            0x1 => { // 1NNN: Jump to location nnn.
                ProgramCounter::Jump(cycle.nnn)
            }
            0x2 => { // 2NNN: Calls subroutine at address NNN
                // @TODO: Implement
                ProgramCounter::Unknown
            }
            0x3 => { // 3XKK Skip next instruction if Vx = kk.
                ProgramCounter::skip_when(self.v[cycle.x as usize] == cycle.get_kk())
            }
            0x4 => { // 4XKK Skip next instruction if NOT Vx = kk.
                ProgramCounter::skip_when(self.v[cycle.x as usize] != cycle.get_kk())
            }
            0x5 => { // 5xy0: Skip next instruction if Vx = Vy.
                ProgramCounter::skip_when(self.v[cycle.x as usize] == self.v[cycle.y as usize])
            }
            0x6 => { // 6xkk: Set Vx = kk
                self.v[cycle.x as usize] = cycle.get_kk();
                ProgramCounter::Next
            }
            0x7 => { // 7xkk: Vx = Vx + kk.
                self.v[cycle.x as usize] += cycle.get_kk();
                ProgramCounter::Next
            }
            _ => panic!("Unknown opcode: {}", self.opcode)
        };

        self.pc = next_pc.resolve();
    }
}

fn main() {
    let mut engine = Engine {
        memory: [0; 4096],
        opcode: 0,
        v: [0; 16],
        i: 0,
        pc: 0x200,
        gfx: [0; 64 * 32],
        delay_timer: 0 as char,
        sound_timer: 0 as char,
        stack: [0; 16],
        stackpointer: 0,
        key: [0 as char; 16],
        draw_flag: false,
    }; // This looks so wrong, is this wrong, I think this is wrong. 
    // From what I understood I have to init my struct like this because the nature of Rust doesn't allow uninit values in safe mode
    // Please somebody give me a better solution, it feels so freaking wrong

    engine.read_game("TETRIS");

    engine.read_font();

    for _ in 1..50 {
        engine.cycle();
    }
}
