import {default as init} from './game.js';

window.onload = async () => {
    await init('./game_bg.wasm');
}
