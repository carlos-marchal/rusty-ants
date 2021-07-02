//

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
function paint_edge(from, to, color = "rgb(0, 0, 0)") {
  const { x: from_x, y: from_y } = to_canvas_coords(from);
  const { x: to_x, y: to_y } = to_canvas_coords(to);
  canvas_context.beginPath();
  canvas_context.moveTo(from_x, from_y);
  canvas_context.lineTo(to_x, to_y);
  canvas_context.strokeStyle = color;
  canvas_context.stroke();
}

/** @type {HTMLInputElement} */
const number_cities = document.querySelector("#number_cities");
/** @type {HTMLInputElement} */
const show_pheromones = document.querySelector("#show_pheromones");
/** @type {HTMLButtonElement} */
const start_button = document.querySelector("#start_button");
/** @type {HTMLFormElement} */
const controls = document.querySelector("#controls");

controls.addEventListener("submit", async (event) => {
  event.preventDefault();
  number_cities.disabled = true;
  show_pheromones.disabled = true;
  start_button.disabled = true;
  const worker = new Worker("/demo/worker.js");
  function nextResult() {
    return new Promise((resolve) =>
      worker.addEventListener("message", (event) => resolve(event.data), {
        once: true,
      })
    );
  }
  canvas_context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
  worker.postMessage(Number.parseInt(number_cities.value, 10));
  /** @type {City[]} */
  let cities = await nextResult();
  /** @type { wasm_bindgen.HandlerResult } */
  let result;
  do {
    result = await nextResult();
    canvas_context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
    for (const city of cities) {
      paint_city(city);
    }
    if (show_pheromones.checked) {
      const trails = [];
      for (let i = 0; i < cities.length - 1; ++i) {
        for (let j = i + 1; j < cities.length; ++j) {
          trails.push({ i, j, trail: result.trails[j][i] });
        }
      }
      const trailValues = trails.map(({ trail }) => trail);
      const max = Math.max(...trailValues);
      for (const trail of trails) {
        const alpha = trail.trail / max;
        paint_edge(
          cities[trail.i],
          cities[trail.j],
          `rgba(255, 117, 20, ${alpha})`
        );
      }
    } else {
      for (let i = 1; i < result.tour.length; ++i) {
        paint_edge(cities[result.tour[i - 1]], cities[result.tour[i]]);
      }
    }
  } while (!result.done);
  for (let i = 1; i < result.tour.length; ++i) {
    paint_edge(cities[result.tour[i - 1]], cities[result.tour[i]]);
  }
  number_cities.disabled = false;
  show_pheromones.disabled = false;
  start_button.disabled = false;
});
number_cities.disabled = false;
show_pheromones.disabled = false;
start_button.disabled = false;
