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
                    div (style="display: block; position: relative;", class="min-h-screen min-w-screen") {
                        div (style="position: absolute; width: 100%; height: 100%;", class="bg-purple-400") {}
                        div (style="position: relative;", class="min-h-screen min-w-screen flex justify-center items-center") {
                            h1 (class="items-center text-8xl text-red-800 font-bold") { "Learning Rust" }
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
