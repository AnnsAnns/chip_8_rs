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


fn main() {
    let mut engine = Engine::new(false);


    engine.read_font();
    engine.read_game("TETRIS");

    loop {
        engine.cycle();

        if engine.waiting_for_input {
            panic!("Waiting for input - Not implemented yet")
        }

        if engine.waiting_for_draw && engine.draw_flag {
            panic!("Waiting for screen to be drawn - Not implemented yet")
        }
    }
}
