use log::info;
use sycamore::{prelude::*, rt::JsCast};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Document, Event};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("ra Ra RA RENDEERRRRRRRRRRRRRR!!!....");

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
    info!("Renderizationingator complete. Enjoy your webpage sir (Tips hat).");
}

#[derive(Props)]
struct PresentationProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
fn Presentation<'a, G: Html>(cx: Scope<'a>, props: PresentationProps<'a, G>) -> View<G> {
    // Component State
    let slide_anchor_counter = create_signal(cx, SlideAnchorCounter(0));
    let slide_refs = create_rc_signal(SlideRefs::<G>::new());
    let current_slide_id = create_rc_signal(Option::<String>::None);

    // Clones for context.
    let slide_refs_context_clone = RcSignal::clone(&slide_refs);

    // Make some signals available in the global context.
    provide_context_ref(cx, slide_anchor_counter);
    provide_context(cx, slide_refs_context_clone);

    // Render children now.
    let children = props.children.call(cx);

    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let slide_refs_event_clone = RcSignal::clone(&slide_refs);
    let current_slide_id_event_clone = RcSignal::clone(&current_slide_id);
    let event_listener = gloo::events::EventListener::new(&document, "keydown", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        let key = event.key();
        let slide_elements: Vec<web_sys::HtmlElement> = slide_refs_event_clone
            .get()
            .0 // Vec<NodeRef<G>>
            .iter()
            .map(|node_ref| {
                node_ref
                    .get::<DomNode>()
                    .to_web_sys()
                    .value_of()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap_throw()
            })
            .collect();
        // If the current ID has not been set (app just started), set the current as the first.
        let current_id = match current_slide_id_event_clone.get().as_ref() {
            Some(id) => id.clone(),
            None => {
                if let Some(element) = slide_elements.first() {
                    element.id().clone()
                } else {
                    // There are no slides.
                    return;
                }
            }
        };
        match key.as_str() {
            "ArrowRight" => {
                let mut iter = slide_elements.iter();
                iter.find(|element| element.id() == current_id);
                if let Some(target_element) = iter.next() {
                    target_element.scroll_into_view();
                }
                // otherwise we must be at the last slide.
            }
            "ArrowLeft" => {
                let mut iter = slide_elements.iter().rev();
                iter.find(|element| element.id() == current_id);
                if let Some(target_element) = iter.next() {
                    target_element.scroll_into_view();
                }
                // otherwise we must be at the first slide.
            }
            _ => (), // A different key was pressed.
        }
    });
    // Bind the event_listener to the context so that it is not dropped.
    let _ = create_signal(cx, event_listener);

    let onscroll = move |_event: Event| {
        // Unpack the references to the Slide nodes.
        // let slide_refs = RcSignal::clone(&slide_refs_clone);
        let slide_refs = &slide_refs.get().0;

        // Get the element ID of the highest Slide where the top is under the top of the window.
        let some_current_id = slide_refs
            .iter()
            .map(|node_ref| {
                let element = node_ref
                    .get::<DomNode>()
                    .to_web_sys()
                    .value_of()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap_throw();
                let id = element.id();
                let top = element.get_bounding_client_rect().top();
                (id, top)
            })
            .filter(|(_, top)| *top >= 0.0)
            .map(|(id, _)| id)
            .next();

        // Should always return an ID unless there are last slide is taller than the viewport.
        // In which case it is none.
        current_slide_id.set(some_current_id);
    };

    let style = r#"
        scroll-snap-type: mandatory;
        scroll-snap-points-y: repeat(300px);
        scroll-snap-type: y mandatory;
        overflow-y: scroll;
    "#;

    view! { cx,
        div (class="max-h-screen min-w-screen snap-proximity snap-start", style=style, on:scroll=onscroll)
        {
            (children)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SlideAnchorCounter(usize);

#[derive(Clone, PartialEq, Eq, Debug)]
struct SlideRefs<G: Html>(Vec<NodeRef<G>>);

impl<G: Html> SlideRefs<G> {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Props)]
struct SlideProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a, G>) -> View<G> {
    // Determine the id anchor of this component.
    let slide_anchor_counter = use_context::<Signal<SlideAnchorCounter>>(cx);
    let this_slide_anchor_number = slide_anchor_counter.get().0;
    slide_anchor_counter.set(SlideAnchorCounter(this_slide_anchor_number + 1));
    let id = format!("_target_slide_{this_slide_anchor_number}");

    // Add a reference to the Instanciated component to the Presentation context.
    let node_ref = create_node_ref(cx);
    let slide_refs = use_context::<RcSignal<SlideRefs<G>>>(cx);
    let mut extended_slide_refs = slide_refs.get().0.clone();
    extended_slide_refs.push(node_ref.clone());
    slide_refs.set(SlideRefs(extended_slide_refs));

    // Render children.
    let children = props.children.call(cx);

    // Render this component.
    view! {cx,
        section (ref=node_ref, id=id, class="min-h-screen min-w-screen bg-white", style="scroll-snap-align: start;")
        {
            (children)
        }
    }
}
