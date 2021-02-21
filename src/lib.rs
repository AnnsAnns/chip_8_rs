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

mod engine;
mod program_counter;
mod font;

use engine::Engine;

const HEIGHT: usize = 32;
const WIDTH: usize = 64;

pub struct Js {
    pub engine: Engine
}

impl Js {
    #[no_mangle]
    pub fn new() -> Self {
        Self {
            engine: Engine::new(false),
        }
    }

    #[no_mangle]
    fn engine_init(&mut self, file: &str) {
        self.engine.read_font();
        self.engine.read_game(file);
    }

    #[no_mangle]
    pub fn engine_cycle(&mut self) {
        self.engine.cycle();
    }

    #[no_mangle]
    pub fn send_input(&mut self, input_key: u8) {
        self.engine.pressed_key = input_key;
        self.engine.waiting_for_input = false;
    }
}