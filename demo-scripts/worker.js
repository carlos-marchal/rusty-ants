/// <reference lib="webworker" />
importScripts("/pkg/rusty_ants.js");

self.addEventListener("message", async (event) => {
  await wasm_bindgen("../pkg/rusty_ants_bg.wasm");
  const { SolveHandler } = wasm_bindgen;
  /** @type {number} */
  const n = event.data;
  const solver = new SolveHandler(n, 250);
  self.postMessage(solver.cities);
  /** @type {wasm_bindgen.HandlerResult} */
  let result;
  do {
    result = solver.run();
    self.postMessage(result);
  } while (!result.done);
});
