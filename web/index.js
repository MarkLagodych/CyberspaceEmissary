// CyberspaceEmissary, a terminal game
// Copyright 2022 Mark Lagodych
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


import { default as init_wasm, GameRunner } from './wasm/wasm.js';


var game_runner;

async function load_wasm() {
    await init_wasm();
    game_runner = new GameRunner();
}
await load_wasm();

var pressed_keys = new Set;
window.my_interval_object = null;

window.onload = ()=>{
    window.my_interval_object = setInterval(update, 30);

    window.onkeydown = (key_event)=>{
        pressed_keys.add(key_event.code);
    };

    window.onkeyup = (key_event)=>{
        if (pressed_keys.has(key_event.code)) {
            pressed_keys.delete(key_event.code);
        }
    };

    new Audio('./web/assets/music.mp3').play();
};

function update() {
    pressed_keys.forEach((key) => {
        if (game_runner.is_expecting_text()) {
            if (key == 'Space' || key.startsWith('Key') || key == 'Backspace')
                pressed_keys.delete(key); // prevent from continuous handling of a single key
        }
        
        switch (key) {
            case 'ArrowRight':  game_runner.handle_key('d'); break;
            case 'ArrowLeft':   game_runner.handle_key('a'); break;
            case 'ArrowUp':     game_runner.handle_key('w'); break;
            case 'ArrowDown':   game_runner.handle_key('s'); break;
            case 'Backspace': game_runner.handle_key('`'); break;
            case 'Enter': game_runner.handle_key('\n'); break;
            case 'Space': game_runner.handle_key(' '); break;
            default: {
                if (key.startsWith("Key"))
                    game_runner.handle_key(key.slice(3).toLowerCase());
                break;
            }
        }
    });

    game_runner.update();

    if (game_runner.has_stopped()) {
        clearInterval(window.my_interval_object);
        window.my_interval_object = null;
    }
}