use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::generate_custom_mesh::{run_bevy_app, send_reset_rotation};

mod generate_custom_mesh;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

// declare the assets here in main.rs because for some reason calling asset!() in other modules is
// giving errors about how `manganis` can't be found as a crate. !?
pub const ARRAY_TEXTURE: Asset = asset!("assets/textures/array_texture.png");

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "width: 100vw; height: 100vh; margin: 0; padding: 0;",

            // Dioxus UI overlay
            div {
                style: "position: absolute; top: 12px; left: 12px; z-index: 100; color: white; font-family: monospace; background: rgba(0,0,0,0.7); padding: 10px; border-radius: 5px;",
                "Dioxus + Bevy 3D Demo"
                br {}
                "Controls: Space: Change UVs, X/Y/Z: Rotate, R: Reset"
                br {}
                button {
                    onclick: move |_|  send_reset_rotation(),
                    style: "margin-top: 10px; padding: 5px 10px; background: #4CAF50; color: white; border: none; border-radius: 3px; cursor: pointer;",
                    "Reset Rotation (Dioxus)"
                }
            }

            // Canvas for Bevy rendering
            BevyExample {}
        }
    }
}

#[component]
fn BevyExample() -> Element {
    use_effect(move || {
        spawn_local(async {
            run_bevy_app().await;
        });
    });

    rsx! {
        canvas {
            id: "bevy-canvas",
            style: "width: 100%; height: 100%; display: block;",
        }
    }
}
