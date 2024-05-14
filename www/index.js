import { FractalGenerator } from "wasm-project";
import { memory } from "wasm-project/wasm_project_bg";
import positions from './positions.js';


const WIDTH = 800;
const HEIGHT = 800;
const canvas = document.getElementById("fractal-canvas");

function get_view_ranges() {
    const rstartInput = document.getElementById("rstart");
    const rendInput = document.getElementById("rend");
    const istartInput = document.getElementById("istart");
    const iendInput = document.getElementById("iend");

    return [parseFloat(rstartInput.value), parseFloat(rendInput.value), parseFloat(istartInput.value), parseFloat(iendInput.value)];
}

const fractal_generator = (() => {
    const [rstart, rend, istart, iend] = get_view_ranges();
    return FractalGenerator.new(rstart, rend, istart, iend, 1000, WIDTH, HEIGHT);
})();

canvas.height = WIDTH;
canvas.width = HEIGHT;
const ctx = canvas.getContext('2d');

function update_canvas() {
    fractal_generator.generate();
    const bitmap = fractal_generator.raw_pixels();
    const pixels = new Uint8ClampedArray(memory.buffer, bitmap, WIDTH * HEIGHT * 4);
    let imageData = new ImageData(pixels, WIDTH)
    ctx.putImageData(imageData, 0, 0);
};

update_canvas();

let positionIndex = 0;
let showNextFrame = true;
let intervalId;

function slideshow() {
    const frameTime = 5000;
    let renderTime;
    if (showNextFrame) {
        const [rstart, rend, istart, iend] = positions[positionIndex++];
        if (positionIndex >= positions.length) {
            positionIndex = 0;
        }
        document.getElementById("rstart").value = rstart;
        document.getElementById("rend").value = rend;
        document.getElementById("istart").value = istart;
        document.getElementById("iend").value = iend;
        const start = performance.now();
        fractal_generator.move_view(rstart, rend, istart, iend);
        update_canvas();
        renderTime = performance.now() - start;
        showNextFrame = false;
    }

    const timeout = Math.max(frameTime - renderTime, 0);
    intervalId = setTimeout(() => {
        showNextFrame = true;
        slideshow();
    }, timeout);
}

document.getElementById("startBtn").addEventListener("click", () => {
    slideshow();
    document.getElementById("startBtn").disabled = true;
    document.getElementById("stopBtn").disabled = false;
    document.getElementById("axes").querySelectorAll("input").forEach(input => input.disabled = true);
});

document.getElementById("stopBtn").addEventListener("click", () => {
    clearInterval(intervalId);
    showNextFrame = true;
    document.getElementById("startBtn").disabled = false;
    document.getElementById("stopBtn").disabled = true;
    document.getElementById("axes").querySelectorAll("input").forEach(input => input.disabled = false);
});

document.getElementById("axes").addEventListener("submit", event => {
    event.preventDefault();
    clearInterval(intervalId);
    const [rstart, rend, istart, iend] = get_view_ranges();

    fractal_generator.move_view(rstart, rend, istart, iend);
    update_canvas();
});

document.addEventListener("keydown", event => {
    if (event.key === "Escape") {
        showNextFrame = true;
        clearInterval(intervalId);
        document.getElementById("startBtn").disabled = false;
        document.getElementById("stopBtn").disabled = true;
        document.getElementById("axes").querySelectorAll("input").forEach(input => input.disabled = false);
    }
});
