import {default as fillTable} from "./score-table.js"

window.onload = async () => {
    await fillTable(document.getElementById("score_table"))
}