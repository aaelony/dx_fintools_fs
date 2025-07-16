use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};
use num::Float;
use num_format::{Locale, ToFormattedString};
use tracing::debug;
use tracing_subscriber;

mod compounding;
use compounding::{compute_fv, Compounding};

/// Validates numeric input and updates the corresponding signals
/// Returns true if the input is valid, false otherwise
fn validate_numeric_input(
    cleaned_input: &str,
    validity_signal: &mut Signal<bool>,
    value_signal: &mut Signal<f64>,
) -> bool {
    match cleaned_input.parse::<f64>() {
        Ok(value) if value > 0.0 && value.is_finite() => {
            validity_signal.set(true);
            value_signal.set(value);
            true
        }
        Ok(_) | Err(_) => {
            validity_signal.set(false);
            false
        }
    }
}

/// Generates appropriate error message for invalid numeric input
fn get_numeric_error_message(input_text: &str, field_name: &str) -> String {
    let cleaned_input = input_text.replace(",", "").replace(" ", "");
    if let Ok(value) = cleaned_input.parse::<f64>() {
        if value <= 0.0 {
            format!("{} must be greater than zero", field_name)
        } else {
            "Invalid number format".to_string()
        }
    } else {
        "Please enter a valid number (digits and decimal point only)".to_string()
    }
}

const COMPOUNDING_OPTIONS: &[(Compounding, &str, &str)] = &[
    (Compounding::Annual, "annual", "Annual"),
    (Compounding::Semiannually, "semiannual", "Semi-annually"),
    (Compounding::Quarterly, "quarterly", "Quarterly"),
    (Compounding::Monthly, "monthly", "Monthly"),
    (Compounding::Weekly, "weekly", "Weekly"),
    (Compounding::Daily, "daily", "Daily"),
];

#[component]
fn NumericInput(
    label: String,
    placeholder: String,
    input_signal: Signal<String>,
    value_signal: Signal<f64>,
    valid_signal: Signal<bool>,
    field_name: String,
    css_prefix: String,
) -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; margin-bottom: 15px;",
            label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
                {label}
            }
            input {
                placeholder: placeholder,
                value: "{input_signal}",
                class: if valid_signal() {
                    format!("{}-input-valid", css_prefix)
                } else {
                    format!("{}-input-invalid", css_prefix)
                },
                style: {
                    let input_width = std::cmp::max(100, input_signal().len() * 9 + 20);
                    format!("padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;", input_width)
                },
                oninput: move |event| {
                    let input_text = event.value();
                    input_signal.set(input_text.clone());
                    if input_text.trim().is_empty() {
                        valid_signal.set(true);
                        return;
                    }
                    let cleaned_input = input_text.replace(",", "").replace(" ", "");
                    validate_numeric_input(&cleaned_input, &mut valid_signal, &mut value_signal);
                },
            }
        }
        if !valid_signal() && !input_signal().trim().is_empty() {
            div { style: "color: #ff0000; font-size: 12px; margin-left: 160px; margin-bottom: 10px;",
                {get_numeric_error_message(&input_signal(), &field_name)}
            }
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/fv-calculator")]
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
fn FutureValueUI() -> Element {
    let mut current_value = use_signal(|| 0.03875);
    let interest_rate = current_value();
    let mut years_signal = use_signal(|| 7.0);
    let mut periods_per_year_signal = use_signal(|| Compounding::Annual);
    let mut principal_signal = use_signal(|| 1000.00 as f64);
    let mut amount_input_valid = use_signal(|| true);
    let mut principal_input = use_signal(|| "1000.00".to_string());
    let mut years_input = use_signal(|| "7.0".to_string());
    let mut years_input_valid = use_signal(|| true);
    // let principal_amount = 10_000.0f64;
    let principal_amount = principal_signal();
    let years: f64 = years_signal();
    let periods_per_year = periods_per_year_signal().periods_per_year();

    let fv = compute_fv(principal_amount, interest_rate, periods_per_year, years);

    let fv_dollars = (fv as i64).to_formatted_string(&num_format::Locale::en);
    let fv_cents = (fv * 100.0) as i64 % 100;
    let fv = format!("{}.{:02}", fv_dollars, fv_cents);

    let periods_string = periods_per_year_signal().to_string();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/slider.css") }
        hr {}

        // Compounding period dropdown
        div { style: "display: flex; align-items: center; margin-bottom: 15px;",
            label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
                "Compounding Period:"
            }
            select {
                style: {
                    let dropdown_width = 150;
                    format!(
                        "border: 1px solid #ccc; background: gray; padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace; ",
                        dropdown_width,
                    )
                },
                onchange: move |event| {
                    let value = event.value();
                    if let Some(&(compounding, _, _)) = COMPOUNDING_OPTIONS
                        .iter()
                        .find(|(_, value_str, _)| *value_str == value.as_str())
                    {
                        periods_per_year_signal.set(compounding);
                    }
                },
                {
                    COMPOUNDING_OPTIONS
                        .iter()
                        .map(|(compounding, value, display)| {
                            rsx! {
                                option {
                                    value: *value,
                                    selected: matches!(periods_per_year_signal(), comp if comp == *compounding),
                                    style: if *value == "annual" { "background: gray; color: white;" } else { "" },
                                    {*display}
                                }
                            }
                        })
                }
            }

        }

        // -------------------------------------------------------------------
        // Input Principal
        NumericInput {
            label: "Principal Amount ($):".to_string(),
            placeholder: "Enter initial principal amount (e.g., 10000.00)".to_string(),
            input_signal: principal_input,
            value_signal: principal_signal,
            valid_signal: amount_input_valid,
            field_name: "Principal amount".to_string(),
            css_prefix: "principal".to_string(),
        }



        // div { style: "display: flex; align-items: center; margin-bottom: 15px;",
        //     label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
        //         "Principal Amount ($):"
        //     }
        //     input {
        //         placeholder: "Enter initial principal amount (e.g., 10000.00)",
        //         value: "{principal_input}",
        //         class: if amount_input_valid() { "principal-input-valid" } else { "principal-input-invalid" },
        //         style: {
        //             let input_width = std::cmp::max(100, principal_input().len() * 9 + 20);
        //             format!("padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;", input_width)
        //         },
        //         oninput: move |event| {
        //             let input_text = event.value();
        //             principal_input.set(input_text.clone());
        //             if input_text.trim().is_empty() {
        //                 amount_input_valid.set(true);
        //                 return;
        //             }
        //             let cleaned_input = input_text.replace(",", "").replace(" ", "");
        //             validate_numeric_input(&cleaned_input, &mut amount_input_valid, &mut principal_signal);
        //         },
        //     }
        // }
        // if !amount_input_valid() && !principal_input().trim().is_empty() {
        //     div { style: "color: #ff0000; font-size: 12px; margin-left: 160px; margin-bottom: 10px;",
        //     {get_numeric_error_message(&principal_input(), "Principal amount")}

        //     }
        // }

        // -------------------------------------------------------------------
        // Input Years
        NumericInput {
            label: "Number of Years:".to_string(),
            placeholder: "Enter number of years (e.g. 5.0)".to_string(),
            input_signal: years_input,
            value_signal: years_signal,
            valid_signal: years_input_valid,
            field_name: "Number of years".to_string(),
            css_prefix: "years".to_string(),
        }
        // div { style: "display: flex; align-items: center; margin-bottom: 15px;",
        //     label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
        //         "Number of Years:"
        //     }
        //     input {
        //         placeholder: "Enter number of years (e.g. 5.0)",
        //         value: "{years_input}",
        //         class: if years_input_valid() { "years-input-valid" } else { "years-input-invalid" },
        //         style: {
        //             let input_width = std::cmp::max(80, years_input().len() * 9 + 20);
        //             format!(
        //                 "padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;",
        //                 input_width,
        //             )
        //         },
        //         oninput: move |event| {
        //             let input_text = event.value();
        //             years_input.set(input_text.clone());
        //             if input_text.trim().is_empty() {
        //                 years_input_valid.set(true);
        //                 return;
        //             }
        //             let cleaned_input = input_text.replace(",", "").replace(" ", "");
        //             validate_numeric_input(&cleaned_input, &mut years_input_valid, &mut years_signal);
        //         },
        //     }
        // }
        // if !years_input_valid() && !years_input().trim().is_empty() {
        //     div { style: "color: #ff0000; font-size: 12px; margin-left: 160px; margin-bottom: 10px;",
        //        {get_numeric_error_message(&years_input(), "Number of years")}
        //     }
        // }

        // Input slider for interest rate
        div { style: "color: #333; font-weight: bold;",
            "Interest Rate:"
            Slider {
                class: "slider",
                label: "Interest Rate Slider",
                horizontal: true,
                min: 0.0,
                max: 50.0,
                step: 0.01,
                default_value: SliderValue::Single(3.875),
                on_value_change: move |value: SliderValue| {
                    let SliderValue::Single(v) = value;
                    current_value.set(v / 100.0);
                },
                SliderTrack { class: "slider-track",
                    SliderRange { class: "slider-range" }
                    SliderThumb { class: "slider-thumb" }
                }
            }
        }
        br {}
        div {
            id: "FutureValueCalculationConfig",
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            "{periods_string} Future value of {principal_amount} at {interest_rate * 100.0:.3}% for {years} years: "
        }
        div {
            id: "FutureValueCalculation",
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            " ${fv}"
        }
    }
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

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        // Hero {}

        hr {}
        br {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            //Link { to: Route::Home {}, "Home" }
            Link { to: Route::FutureValueUI {}, "Future Value Calculator" }
                //Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}

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
