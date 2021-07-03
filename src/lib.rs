mod cities;
mod edges;
mod instance;
mod utils;

use js_sys::Date;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
export interface City {
  x: number;
  y: number;
}

export type HandlerResult = {
  done: boolean;
  tour: number[];
  tour_length: number;
};

export class SolveHandler {
  constructor(cities: City[], update_interval?: number);
  run(): HandlerResult;
}
"#;

#[wasm_bindgen]
pub struct SolveHandler {
    instance: instance::Instance,
    update_interval: f64,
    next_update: f64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HandlerResult {
    done: bool,
    tour: Vec<usize>,
    tour_length: f32,
}

#[wasm_bindgen]
impl SolveHandler {
    #[wasm_bindgen(skip_typescript, constructor)]
    pub fn new(cities: JsValue, update_interval: Option<f64>) -> Self {
        utils::set_panic_hook();
        let cities: Vec<cities::City> = from_value(cities).unwrap();
        SolveHandler {
            instance: instance::Instance::new(&cities),
            update_interval: update_interval.unwrap_or(100.0),
            next_update: 0.0,
        }
    }

    #[wasm_bindgen(skip_typescript)]
    pub fn run(&mut self) -> JsValue {
        let mut improved = true;
        while Date::now() < self.next_update && improved {
            improved = self.instance.try_improvement();
        }
        let solution = self.instance.get_solution();
        self.next_update = Date::now() + self.update_interval;
        to_value(&HandlerResult {
            done: !improved,
            tour: solution.tour,
            tour_length: solution.tour_length,
        })
        .unwrap()
    }
}
