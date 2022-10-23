use log::info;
use sycamore::{prelude::*, rt::JsCast};
use wasm_bindgen::UnwrapThrowExt;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Page loaded");

    sycamore::render(|cx| {
        view! { cx,
            Presentation {
                Slide {
                    div (class="min-h-screen min-w-screen bg-yellow-300") {}
                }
                Slide {
                    div (class="min-h-screen min-w-screen bg-blue-300") {}
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
    info!("end");
}

#[derive(Props)]
struct PresentationProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
fn Presentation<'a, G: Html>(cx: Scope<'a>, props: PresentationProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let style = r#"
        scroll-snap-type: mandatory;
        scroll-snap-points-y: repeat(300px);
        scroll-snap-type: y mandatory;
        overflow-y: scroll;
    "#;

    let current_slide = create_rc_signal(0u32);

    let document = web_sys::window().unwrap().document().unwrap();
    let event_listener = gloo::events::EventListener::new(&document, "keydown", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let key = event.key();
        info!("Key was pressed: {key}");
    });
    // Bind the event_listener to the context so that it is not dropped.
    let _ = create_signal(cx, event_listener);

    // Add anchors to all Slides.
    if let Some(views) = children.as_fragment() {
        for (i, view) in views.iter().enumerate() {
            let element = view.as_node().unwrap();
            if element.to_web_sys().node_name() == "SECTION" {
                // TODO: this if statement only slightly safeguards the selection of Slide()s.
                // In the future, use a way to filter out anything that is not a Slide.
                element.set_attribute("id", &format!("_target_slide_{i}"));
            }
        }
    }

    view! { cx,
        div (class="max-h-screen min-w-screen snap-proximity snap-start", style=style) {
            (children)
        }
    }
}

#[derive(Props)]
struct SlideProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! {cx,
        section (class="min-h-screen min-w-screen bg-red-300", style="scroll-snap-align: start;") {
            (children)
        }
    }
}
