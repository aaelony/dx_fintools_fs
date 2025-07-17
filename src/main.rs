use dioxus::prelude::*;

use tracing::debug;
use tracing_subscriber;

mod blog;
mod compounding;
mod future_value_component;
mod numeric_input_component;

use blog::Blog;
use future_value_component::FutureValueUI;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route { // components in the enum are called and MUST exist.
    #[layout(Navbar)]
    #[route("/")]
    //Home {},
    //#[route("/fv-calculator")]
    FutureValueUI  {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        document::Meta { charset: "utf-8" }

        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        // Hero {}

        hr {}
        br {}

        div {
            p { "Hello! My name is Avram. " }
        }
        div { "The following site is designed to showcase use of the Dioxus Rust library." }
        div {
            br {}
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            // Link { to: Route::Home {}, "Home" }
            Link { to: Route::FutureValueUI {}, "Future Value Calculator" }
                //Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}

// #[component]
// pub fn Hero() -> Element {
//     rsx! {
//         div { id: "hero",
//             img { src: HEADER_SVG, id: "header" }
//             div { id: "links",
//                 a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
//                 a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
//                 a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
//                 a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
//                 a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
//                     "💫 VSCode Extension"
//                 }
//                 a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
//             }
//         }
//     }
// }

// Echo component that demonstrates fullstack server functions.
// #[component]
// fn Echo() -> Element {
//     let mut response = use_signal(|| String::new());

//     rsx! {
//         div { id: "echo",
//             h4 { "ServerFn Echo" }
//             input {
//                 placeholder: "Type here to echo...",
//                 oninput: move |event| async move {
//                     let data = echo_server(event.value()).await.unwrap();
//                     response.set(data);
//                 },
//             }

//             if !response().is_empty() {
//                 p {
//                     "Server echoed: "
//                     i { "{response}" }
//                 }
//             }
//         }
//     }
// }

// Echo the user input on the server.
// #[server(EchoServer)]
// async fn echo_server(input: String) -> Result<String, ServerFnError> {
//     Ok(input)
// }
