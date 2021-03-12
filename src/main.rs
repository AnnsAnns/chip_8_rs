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
extern crate minifb;
extern crate std;

use minifb::Window;
use minifb::WindowOptions;
use minifb::Key;

use std::time::Duration;

mod engine;
mod program_counter;
mod font;

use engine::Engine;

const HEIGHT: usize = 32;
const WIDTH: usize = 64;


fn main() {
    let mut engine = Engine::new(false);

    let mut window = Window::new(
        "chip_8_rs",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..Default::default()
        }
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_secs_f32(0.5)));

    engine.read_font();
    engine.read_game("TETRIS");

    loop {
        engine.cycle();

        let keys = window.get_keys().unwrap();

        for key in keys {
            use minifb::Key::*;

            engine.input_received = true;
            engine.pressed_key = match key {
                Key1 => 0x0,
                Key2 => 0x1,
                Key3 => 0x2,
                Key4 => 0x3,
                Q => 0x4,
                W => 0x5,
                E => 0x6,
                R => 0x7,
                A => 0x8,
                S => 0x9,
                D => 0xA,
                F => 0xB,
                Z => 0xC,
                X => 0xD,
                C => 0xE,
                V => 0xF,
                _ => {
                    engine.input_received = false;
                    0x0 // Dummy
                }
            }
        }

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                buffer.push(engine.gfx[x][y] as u32)
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if window.is_key_down(Key::Escape) {
            break
        }
    }
}
