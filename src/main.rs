use std::cell::RefCell;

use log::info;
use sycamore::prelude::*;

mod components;
use components::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("ra Ra RA RENDEERRRRRRRRRRRRRR!!!....");

    sycamore::render(|cx| {
        view! { cx,
            Presentation {
                // Slide {
                //     div (style="display: block; position: relative;", class="min-h-screen min-w-screen") {
                //         div (style="position: absolute; width: 100%; height: 100%;", class="bg-slate-100") {
                //             div (style="height: 60%;", class="bg-slate-100")
                //             div (style="height: 10%;", class="bg-black")
                //             div (style="height: 30%;", class="bg-green-600")
                //         }
                //         div (style="position: relative;", class="min-h-screen min-w-screen flex justify-center items-center") {
                //             h1 (class="items-center text-8xl text-red-800 font-bold") { "Learning Rust" }
                //         }
                //     }
                // }
                Slide {
                    div (class="min-h-screen min-w-screen bg-yellow-100 flex justify-center items-center") {
                        div (class="items-center") {
                            ChartOfNoCompromise ()
                        }
                    }
                }
                Slide {
                    div (style="background-color: #16203c; --tw-bg-opacity: 1;", class="min-h-screen min-w-screen flex justify-center items-center") {
                        iframe (style="width: 80vw; height: 80vh;", class="rounded-lg", src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20serde%3A%3A%7BSerialize%2C%20Deserialize%7D%3B%0A%0A%23%5Bderive(Serialize%2C%20Deserialize%2C%20Debug)%5D%0Astruct%20Point%20%7B%0A%20%20%20%20x%3A%20i32%2C%0A%20%20%20%20y%3A%20i32%2C%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20point%20%3D%20Point%20%7B%20x%3A%201%2C%20y%3A%202%20%7D%3B%0A%0A%20%20%20%20%2F%2F%20Convert%20the%20Point%20to%20a%20JSON%20string.%0A%20%20%20%20let%20serialized%20%3D%20serde_json%3A%3Ato_string(%26point).unwrap()%3B%0A%0A%20%20%20%20%2F%2F%20Prints%20serialized%20%3D%20%7B%22x%22%3A1%2C%22y%22%3A2%7D%0A%20%20%20%20println!(%22serialized%20%3D%20%7B%7D%22%2C%20serialized)%3B%0A%0A%20%20%20%20%2F%2F%20Convert%20the%20JSON%20string%20back%20to%20a%20Point.%0A%20%20%20%20let%20deserialized%3A%20Point%20%3D%20serde_json%3A%3Afrom_str(%26serialized).unwrap()%3B%0A%0A%20%20%20%20%2F%2F%20Prints%20deserialized%20%3D%20Point%20%7B%20x%3A%201%2C%20y%3A%202%20%7D%0A%20%20%20%20println!(%22deserialized%20%3D%20%7B%3A%3F%7D%22%2C%20deserialized)%3B%0A%7D")
                    }
                }
                Slide {
                    div (class="min-h-screen min-w-screen bg-green-300") {}
                }
                Slide {
                    div (class="min-h-screen min-w-screen bg-red-300") {}
                }
            }
        }
    });

    info!("Renderizationingator complete. Enjoy your webpage sir (Tips hat).");
}

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlCanvasElement;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// fn document() -> web_sys::Document {
//     window()
//         .document()
//         .expect("should have a document on window")
// }

fn element_from_ref<T, G: GenericNode>(node_ref: &sycamore::noderef::NodeRef<G>) -> T
where
    T: Into<web_sys::HtmlElement> + JsCast,
{
    node_ref
        .get::<DomNode>()
        .to_web_sys()
        .value_of()
        .dyn_into::<T>()
        .unwrap_throw()
}

// TODO: [ ] labels
// TODO: [X] Multiple edges
// TODO: [ ] Animation to single point.
// TODO: [ ] Animation of choose two stator.
// TODO: [ ] Variable length.

use nalgebra::*;

#[component]
pub fn ChartOfNoCompromise<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let canvas_ref = create_node_ref(cx);

    on_mount(cx, move || {
        let canvas: HtmlCanvasElement = element_from_ref(&canvas_ref);
        info!("canvas {canvas:?}");

        let w = canvas.width() as f64;
        let h = canvas.height() as f64;
        info!("canvas width and hight = (w: {w}, h: {h})");

        // all shapes must be in this area.
        let boundary_radius = w.min(h) / 2.0;
        let center = Vector2::new(w / 2.0, h / 2.0);

        let properties = ["this", "that", "other"];

        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // Draw bounding area.
        {
            context.begin_path();
            context.set_stroke_style(&JsValue::from_str("#FF2558"));
            context.set_line_width(1.0);
            context
                .ellipse(
                    center.x,
                    center.y,
                    boundary_radius,
                    boundary_radius,
                    0.0,
                    0.0,
                    360.0,
                )
                .unwrap();
            context.stroke();
        }

        // Draw point circles.
        let point_circle_centers: Vec<Vector2<f64>> = (0..properties.len())
            .into_iter()
            .map(|i| i as f64)
            .map(|i| {
                let i = i as f64;
                let percentage_distance_across_boundary_radius: f64 = 0.8;
                let x_ancular_offset =
                    (i * std::f64::consts::PI * 2.0 / properties.len() as f64).sin();
                let y_ancular_offset =
                    (i * std::f64::consts::PI * 2.0 / properties.len() as f64).cos();
                Vector2::from([
                    boundary_radius * percentage_distance_across_boundary_radius * x_ancular_offset,
                    boundary_radius * percentage_distance_across_boundary_radius * y_ancular_offset,
                ])
            })
            .collect();
        for point in point_circle_centers.iter() {
            context.begin_path();
            context.set_stroke_style(&JsValue::from_str("#022558"));
            context.set_line_width(1.0);
            context
                .ellipse(
                    center.x + point.x,
                    center.y - point.y,
                    boundary_radius / 14.0,
                    boundary_radius / 14.0,
                    0.0,
                    0.0,
                    360.0,
                )
                .unwrap();
            context.stroke();
        }

        // Lines between point circles.
        let mut point_circle_centers_copy = point_circle_centers.clone();
        point_circle_centers_copy.rotate_right(1);
        let line_iter = point_circle_centers
            .iter()
            .zip(point_circle_centers_copy.iter())
            .map(|(a, b)| {
                // Bring the points a and b slightly closer to each other.
                let unit_vector_a_to_b = (b - a) / b.metric_distance(a);
                let movement_distance = (boundary_radius / 14.0) + (boundary_radius / 14.0) / 4.0;
                let a_prime = a + unit_vector_a_to_b * movement_distance;
                let b_prime = b - unit_vector_a_to_b * movement_distance;
                return (a_prime, b_prime);
            });
        for (point_start, point_end) in line_iter {
            context.begin_path();
            context.set_stroke_style(&JsValue::from_str("#022558"));
            context.set_line_width(1.0);
            context.move_to(center.x + point_start.x, center.y - point_start.y);
            context.line_to(center.x + point_end.x, center.y - point_end.y);
            context.stroke();
        }

        // Lable Lines
        for point in point_circle_centers.iter() {}

        let targets = point_circle_centers
            .iter()
            .enumerate()
            .filter(|(i, _)| i >= &(properties.len() / 4))
            .map(|(_, point)| point);

        context.begin_path();
        context.set_stroke_style(&JsValue::from_str("#022558"));
        context.set_line_width(1.0);
        context
            .ellipse(w / 2.0, h / 2.0, 30.0, 30.0, 0.0, 0.0, 360.0)
            .unwrap();

        context.stroke();

        use std::rc::Rc;

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut i = 0;

        *g.borrow_mut() = Some(Closure::new(move || {
            if i > 5 {
                info!("all done!");

                // Drop this closure handler.
                let _ = f.borrow_mut().take();
                return;
            }

            i += 1;
            info!("executing closure: {i}");
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    });

    // Render this component.
    view! {cx,
        canvas (ref=canvas_ref, width=700, height=700)
    }
}
