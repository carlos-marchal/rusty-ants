mod cities;
mod cycle;
mod edges;
mod universe;
mod utils;

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
    (x*available_size + padding, y*available_size + padding)
}

const size: f64 = 800.0;
    const radius: f64 = 4.0;


    fn print_city(context: &web_sys::CanvasRenderingContext2d, city: &cities::City ) {
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
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document.create_element("canvas").unwrap().unchecked_into();
    canvas.set_height(size as u32);
    canvas.set_width(size as u32);
    let body = document.body().unwrap();
    let context: web_sys::CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
    
    let cities = cities::generate(40);

    context.set_fill_style(&JsValue::from_str("green"));
    for city in &cities {
        print_city(&context, &city);
    }

    let mut universe = universe::Universe::new(&cities,&Default::default());
    while let Some(result) = universe.cycle() {

    context.set_line_width(2.0);
    context.set_stroke_style(&JsValue::from_str("red"));
    for window in result.shortest_tour.windows(2) {
        let (start_index, end_index) = (window[0], window[1]);
        let (start, end) = (cities[start_index], cities[end_index]);
        let (start_x, start_y) = to_canvas_coords(start.x, start.y);
        let (end_x, end_y) = to_canvas_coords(end.x, end.y);
    context.begin_path();
        context.move_to(start_x, start_y);
        context.line_to(end_x, end_y);
        context.close_path();
        context.stroke();
        log(&format!("start: {:?},{:?}    end {:?},{:?}", start_x, start_y, end_x, end_y));
    }
}

    
    canvas
}
