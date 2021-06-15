mod cities;
mod cycle;
mod edges;
mod universe;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut u = universe::Universe::new(
        100,
        &universe::UniverseParams {
            // α
            trail_importance: 0.0,
            // β
            distance_importance: 0.0,
            // Q
            distance_constant: 0.0,
            // ρ
            trail_decay: 0.0,
            // NC_max
            max_cycles: 10,
        },
    );
    u.cycle();
}
