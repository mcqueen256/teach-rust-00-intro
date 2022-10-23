use log::info;
use sycamore::prelude::*;

mod components;
use components::prelude::*;

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
