mod cities;
mod edges;
mod instance;
mod universe;
mod utils;

use cities::City;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn to_canvas_coords(x: f64, y: f64) -> (f64, f64) {
    let available_size = size - radius;
    let padding = radius / 2.0;
    (x * available_size + padding, y * available_size + padding)
}

const size: f64 = 800.0;
const radius: f64 = 4.0;

fn print_city(context: &web_sys::CanvasRenderingContext2d, city: &cities::City) {
    context.begin_path();
    let (x, y) = to_canvas_coords(city.x, city.y);
    context.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
    context.fill();
}

// fn print_line(context: &web_sys::CanvasRenderingContext2d, a: &cities::City, b: &cities::City ) {
//     context.begin_path();
//     let (start_x, start_y) = to_canvas_coords(a.x, a.y);
//     let (end_x, end_y) = to_canvas_coords(b.x, b.y);
//     context.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
//     context.fill();
// }

#[wasm_bindgen]
pub fn greet() -> web_sys::HtmlCanvasElement {
    utils::set_panic_hook();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        document.create_element("canvas").unwrap().unchecked_into();
    canvas.set_height(size as u32);
    canvas.set_width(size as u32);
    let body = document.body().unwrap();
    let context: web_sys::CanvasRenderingContext2d =
        canvas.get_context("2d").unwrap().unwrap().unchecked_into();

    let cities = cities::generate(30);
    // let cities: Vec<_> = vec![
    //     City { x: 54.0, y: 67.0 },
    //     City { x: 54.0, y: 62.0 },
    //     City { x: 37.0, y: 84.0 },
    //     City { x: 41.0, y: 94.0 },
    //     City { x: 2.0, y: 99.0 },
    //     City { x: 7.0, y: 64.0 },
    //     City { x: 25.0, y: 62.0 },
    //     City { x: 22.0, y: 60.0 },
    //     City { x: 18.0, y: 54.0 },
    //     City { x: 4.0, y: 50.0 },
    //     City { x: 13.0, y: 40.0 },
    //     City { x: 18.0, y: 40.0 },
    //     City { x: 24.0, y: 42.0 },
    //     City { x: 25.0, y: 38.0 },
    //     City { x: 44.0, y: 35.0 },
    //     City { x: 41.0, y: 26.0 },
    //     City { x: 45.0, y: 21.0 },
    //     City { x: 58.0, y: 35.0 },
    //     City { x: 62.0, y: 32.0 },
    //     City { x: 82.0, y: 7.0 },
    //     City { x: 91.0, y: 38.0 },
    //     City { x: 83.0, y: 46.0 },
    //     City { x: 71.0, y: 44.0 },
    //     City { x: 64.0, y: 60.0 },
    //     City { x: 68.0, y: 58.0 },
    //     City { x: 83.0, y: 69.0 },
    //     City { x: 87.0, y: 76.0 },
    //     City { x: 74.0, y: 78.0 },
    //     City { x: 71.0, y: 71.0 },
    //     City { x: 58.0, y: 69.0 },
    // ].iter().map(|city| City {x: city.x /100.0, y: city.y /100.0}).collect();

    context.set_fill_style(&JsValue::from_str("green"));
    for city in &cities {
        print_city(&context, &city);
    }

    let universe = instance::Instance::new(&cities, &Default::default());
    let best_result: Option<instance::CycleResult> = Some(universe.solve());

    context.set_line_width(2.0);
    context.set_stroke_style(&JsValue::from_str("red"));
    for window in best_result.unwrap().shortest_tour.windows(2) {
        let (start_index, end_index) = (window[0], window[1]);
        let (start, end) = (cities[start_index], cities[end_index]);
        let (start_x, start_y) = to_canvas_coords(start.x, start.y);
        let (end_x, end_y) = to_canvas_coords(end.x, end.y);
        context.begin_path();
        context.move_to(start_x, start_y);
        context.line_to(end_x, end_y);
        context.close_path();
        context.stroke();
    }

    canvas
}
