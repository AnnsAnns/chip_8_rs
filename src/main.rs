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

use std::io;
use std::fs;

struct Engine {
    // The Main Engine of the Emulator
    // Mostly taken from my older emulator: https://github.com/tumGER/CHIP-8/blob/master/main.go
    memory: [char; 4096],
    //0x0 - 0x1FF Chip8 Interpreter
	//0x050-0x0A0 Fonts
	//0x200-0xFFF Program and RAM

    opcode: u16,
    v: [char; 16], // CPU register
    i: u16, // Index register
    pc: u16, // Program counter
    gfx: [char; 64 * 32], // Screen

    delay_timer: char,
    sound_timer: char,

    stack: [u8; 16],
    stackpointer: u8,

    key: [char; 16], // Input

    draw_flag: bool, // Disable actually drawing to the screen
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
            self.memory[n] = byte as char;
            println!("{}", self.memory[n] as u8); // Debug

            n += 1; // Is there a better solution?
        }
    }

    fn cycle(&mut self) {
        self.opcode = self.memory[self.pc as usize] as u16 >> 8 | self.memory[self.pc as usize + 1] as u16; // Get next opcode

        // @TODO: Everything lol
    }
}

fn main() {
    let mut engine = Engine {
        memory: [0 as char; 4096],
        opcode: 0,
        v: [0 as char; 16],
        i: 0,
        pc: 0,
        gfx: [0 as char; 64 * 32],
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

    for _ in 1..15 {
        engine.cycle();
    }
}
