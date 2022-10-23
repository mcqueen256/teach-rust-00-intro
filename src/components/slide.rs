use sycamore::prelude::*;

pub mod prelude {
    pub use super::Slide;
    pub use super::SlideAnchorCounter;
    pub use super::SlideRefs;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SlideAnchorCounter(pub usize);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlideRefs<G: Html>(pub Vec<NodeRef<G>>);

impl<G: Html> SlideRefs<G> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Props)]
pub struct SlideProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a, G>) -> View<G> {
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
