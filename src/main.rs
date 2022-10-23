use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
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
    });
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