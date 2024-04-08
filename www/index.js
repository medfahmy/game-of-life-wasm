import { Cell, Grid } from "game-of-life-wasm";
import { memory } from "game-of-life-wasm/game_of_life_wasm_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#ccc";
const DEAD_COLOR = "#fff";
const ALIVE_COLOR = "#000";

const grid = Grid.new();
const width = grid.width();
const height = grid.height();

const canvas = document.getElementById("game-of-life");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, height * (CELL_SIZE + 1) + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo(width * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * j + 1);
    }

    ctx.stroke();
}

function getIndex(row, col) {
    return row * width + col;
}

function drawCells() {
    const cellsPtr = grid.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

function renderLoop() {
    grid.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
