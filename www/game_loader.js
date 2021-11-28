import {default as init} from './game.js';

async function startGame() {
    await init('./game_bg.wasm');
}

window.onload = startGame
