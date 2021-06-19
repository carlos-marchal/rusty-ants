mod cities;
mod edges;
mod instance;
mod universe;
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

export type HandlerInit = City[] | number;

export type HandlerResult = {
  done: boolean;
  length: number;
  tour: number[];
  trails: number[][];
};

export class SolveHandler {
  constructor(init: HandlerInit, update_interval?: number);
  cities: City[];
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
#[serde(untagged)]
pub enum HandlerInit {
    Fixed(Vec<cities::City>),
    Random(usize),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HandlerResult {
    done: bool,
    length: f32,
    tour: Vec<usize>,
    trails: Vec<Vec<f32>>,
}

#[wasm_bindgen]
impl SolveHandler {
    #[wasm_bindgen(skip_typescript, constructor)]
    pub fn new(init: JsValue, update_interval: Option<f64>) -> Self {
        utils::set_panic_hook();
        let init: HandlerInit = from_value(init).unwrap();
        let cities = match init {
            HandlerInit::Fixed(cities) => cities,
            HandlerInit::Random(size) => cities::generate(size),
        };
        SolveHandler {
            instance: instance::Instance::new(&cities, &Default::default()),
            update_interval: update_interval.unwrap_or(200.0),
            next_update: 0.0,
        }
    }

    #[wasm_bindgen(skip_typescript, getter = cities)]
    pub fn get_cities(&self) -> JsValue {
        to_value(&self.instance.cities).unwrap()
    }

    #[wasm_bindgen(skip_typescript)]
    pub fn run(&mut self) -> JsValue {
        let mut result = self.instance.cycle();
        while Date::now() < self.next_update
            && self.instance.cycle_count < self.instance.params.max_cycles
        {
            result = self.instance.cycle();
        }
        self.next_update = Date::now() + self.update_interval;
        let done = self.instance.cycle_count >= self.instance.params.max_cycles;
        to_value(&HandlerResult {
            done,
            length: result.shortest_length,
            tour: result.shortest_tour,
            trails: (0..self.instance.n)
                .map(|i| {
                    (0..i)
                        .chain(i + 1..self.instance.n)
                        .map(|j| self.instance.edges.get(i, j).trail)
                        .collect()
                })
                .collect(),
        })
        .unwrap()
    }
}
