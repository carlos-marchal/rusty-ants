/** @type {HTMLCanvasElement} */
const canvas = document.querySelector("#cities");
const canvas_context = canvas.getContext("2d");

/** @typedef { wasm_bindgen.City } City */
const CANVAS_SIZE = 800;
const POINT_RADIUS = 3;
const SAFE_SIZE = CANVAS_SIZE - POINT_RADIUS;
const PADDING = POINT_RADIUS / 2;

/** @param {City} city */
function to_canvas_coords({ x, y }) {
  return { x: x * SAFE_SIZE + PADDING, y: y * SAFE_SIZE + PADDING };
}

/** @param {City} city */
function paint_city(city) {
  const { x, y } = to_canvas_coords(city);
  canvas_context.beginPath();
  canvas_context.arc(x, y, POINT_RADIUS, 0, 2 * Math.PI);
  canvas_context.fill();
}

/** @param {City} from @param {City} to */
function paint_edge(from, to) {
  const { x: from_x, y: from_y } = to_canvas_coords(from);
  const { x: to_x, y: to_y } = to_canvas_coords(to);
  canvas_context.beginPath();
  canvas_context.moveTo(from_x, from_y);
  canvas_context.lineTo(to_x, to_y);
  canvas_context.stroke();
}

function clear_canvas() {
  canvas_context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
}

/** @type {HTMLInputElement} */
const number_cities = document.querySelector("#number_cities");
/** @type {HTMLButtonElement} */
const start_button = document.querySelector("#start_button");
/** @type {HTMLFormElement} */
const controls = document.querySelector("#controls");
/** @type {HTMLSpanElement} */
const time = document.querySelector("#time");
/** @type {HTMLSpanElement} */
const tour_length = document.querySelector("#tour_length");

/** @type {City[]} */
let cities;
function reset_cities() {
  let n = Number.parseInt(number_cities.value, 10);
  if (Number.isInteger(n)) {
    cities = new Array(n)
      .fill()
      .map(() => ({ x: Math.random(), y: Math.random() }));
    clear_canvas();
    for (const city of cities) {
      paint_city(city);
    }
  }
}
reset_cities();
number_cities.addEventListener("input", reset_cities);

controls.addEventListener("submit", async (event) => {
  event.preventDefault();
  number_cities.disabled = true;
  start_button.disabled = true;
  time.innerText = "0.0";
  tour_length.innerText = "-";
  const worker = new Worker("/demo/worker.js");
  function nextResult() {
    return new Promise((resolve) =>
      worker.addEventListener("message", (event) => resolve(event.data), {
        once: true,
      })
    );
  }
  canvas_context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
  worker.postMessage(cities);
  /** @type { wasm_bindgen.HandlerResult } */
  let result;
  let start = Date.now();
  do {
    result = await nextResult();
    clear_canvas();
    for (const city of cities) {
      paint_city(city);
    }
    for (let i = 1; i < result.tour.length; ++i) {
      paint_edge(cities[result.tour[i - 1]], cities[result.tour[i]]);
    }
    paint_edge(
      cities[result.tour[result.tour.length - 1]],
      cities[result.tour[0]]
    );
    time.innerText = ((Date.now() - start) / 1000).toFixed(1);
    tour_length.innerText = result.tour_length.toFixed(1);
  } while (!result.done);
  number_cities.disabled = false;
  start_button.disabled = false;
});
number_cities.disabled = false;
start_button.disabled = false;
