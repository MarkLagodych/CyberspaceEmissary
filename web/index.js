import {default as init_wasm, GameRunner } from './wasm/wasm.js';


var game_runner;

async function load_wasm() {
    await init_wasm();
    game_runner = new GameRunner();
}
load_wasm();


window.my_interval_object = null;

window.onload = ()=>{
    window.my_interval_object = setInterval(update, 30);
};

function update() {
    game_runner.update();

    if (game_runner.has_stopped()) {
        clearInterval(window.my_interval_object);
        window.my_interval_object = null;
    }
}