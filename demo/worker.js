/// <reference lib="webworker" />
importScripts("/pkg/rusty_tsp.js");

self.addEventListener("message", async (event) => {
  await wasm_bindgen("../pkg/rusty_tsp_bg.wasm");
  const { SolveHandler } = wasm_bindgen;
  /** @type {City[]} */
  const cities = event.data;
  const solver = new SolveHandler(cities);
  /** @type {wasm_bindgen.HandlerResult} */
  let result;
  do {
    result = solver.run();
    self.postMessage(result);
  } while (!result.done);
});
