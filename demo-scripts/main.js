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
const input = document.querySelector("#number_cities");
/** @type {HTMLButtonElement} */
const start_button = document.querySelector("#start_button");

start_button.addEventListener("click", async () => {
  input.disabled = true;
  start_button.disabled = true;
  const worker = new Worker("/demo-scripts/worker.js");
  function nextResult() {
    return new Promise((resolve) =>
      worker.addEventListener("message", (event) => resolve(event.data), {
        once: true,
      })
    );
  }
  canvas_context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
  worker.postMessage(Number.parseInt(input.value, 10));
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
    const trails = [];
    for (let i = 0; i < cities.length - 1; ++i) {
      for (let j = i + 1; j < cities.length; ++j) {
        trails.push({ i, j, trail: result.trails[j][i] });
      }
    }
    const trailValues = trails.map(({ trail }) => trail);
    const max = Math.max(...trailValues);
    const min = Math.min(...trailValues);
    const sum = trailValues.reduce((sum, next) => sum + next);
    for (const trail of trails) {
      const alpha = trail.trail / max;
      paint_edge(
        cities[trail.i],
        cities[trail.j],
        `rgba(240, 172, 0, ${alpha})`
      );
    }
    console.log(max, min, sum);
  } while (!result.done);
  for (let i = 1; i < result.tour.length; ++i) {
    paint_edge(cities[result.tour[i - 1]], cities[result.tour[i]]);
  }
  input.disabled = false;
  start_button.disabled = false;
});
input.disabled = false;
start_button.disabled = false;
