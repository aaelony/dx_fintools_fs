use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};
use num::Float;
use num_format::{Locale, ToFormattedString};

/// Truncates a floating-point number to two decimal places
pub fn truncate_to_two_decimal_places<T: Float>(value: T) -> T {
    (value * T::from(100.0).unwrap()).round() / T::from(100.0).unwrap()
}

// Computes the future value (FV) of an investment, including interest.
///
/// # Parameters:
/// - `initial_value`: Initial principal amount (P)
/// - `annual_interest_rate`: Annual interest rate (r), e.g., 0.04 for 4%
/// - `n_per_year_compounded`: Number of compounding periods per year (n)
/// - `n_years`: Time in years (t)
///
/// # Formula:
/// FV = P * (1 + r/n)^nt
///
/// # Returns:
/// The future value (FV) truncated to two decimal places.
pub fn compute_fv<T>(
    initial_value: T,
    annual_interest_rate: T,
    n_per_year_compounded: T,
    n_years: T,
) -> T
where
    T: Float,
{
    let nt = n_per_year_compounded * n_years;
    let compound_rate = T::one() + annual_interest_rate / n_per_year_compounded;

    truncate_to_two_decimal_places(initial_value * compound_rate.powf(nt))
}

// Computes the present value (PV) of an investment
///
/// # Parameters:
/// - `future_value`: Future amount (FV)
/// - `annual_interest_rate`: Annual interest rate (r), e.g., 0.04 for 4%
/// - `n_per_year_compounded`: Number of compounding periods per year (n)
/// - `n_years`: Time in years (t)
///
/// # Formula:
/// PV = FV / (1 + r/n)^nt
///
/// # Returns:
/// The present value (PV) truncated to two decimal places.
pub fn compute_pv<T>(
    future_value: T,
    annual_interest_rate: T,
    n_per_year_compounded: T,
    n_years: T,
) -> T
where
    T: Float,
{
    let nt = n_per_year_compounded * n_years;
    let compound_rate = T::one() + annual_interest_rate / n_per_year_compounded;

    truncate_to_two_decimal_places(future_value / compound_rate.powf(nt))
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn MySlider() -> Element {
    let mut current_value = use_signal(|| 0.5);
    let interest_rate = truncate_to_two_decimal_places(current_value());
    let mut principal_signal = use_signal(|| 1000.00 as f64);
    let mut principal_input = use_signal(|| "1000.00".to_string());
    let mut input_valid = use_signal(|| true);
    // let principal_amount = 10_000.0f64;
    let principal_amount = principal_signal();
    let fv = compute_fv(principal_amount, 0.05, 12.0, interest_rate);
    let fv_dollars = (fv as i64).to_formatted_string(&num_format::Locale::en);
    let fv_cents = (fv * 100.0) as i64 % 100;
    let fv = format!("{}.{:02}", fv_dollars, fv_cents);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/slider.css") }

        // Display the current value
        div {
            id: "FutureValueCalculation",
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            "Interest rate: {interest_rate}%, FV: ${fv}"
        }

        div {
            label {
                style: "display: block; margin-bottom: 5px; font-weight: bold; color: #333;",
                "Principal Amount ($):"
            }
            input {
                placeholder: "Enter initial principal amount (e.g., 10000.00)",
                value: "{principal_input}",
                style: {
                    let input_width = std::cmp::max(100, principal_input().len() * 9 + 20);
                    if input_valid() {
                        format!("border: 1px solid #ccc; padding: 6px 8px; margin-bottom: 10px; width: {}px; border-radius: 4px; font-family: monospace;", input_width)
                    } else {
                        format!("border: 2px solid #ff0000; padding: 6px 8px; margin-bottom: 10px; background-color: #ffe6e6; width: {}px; border-radius: 4px; font-family: monospace;", input_width)
                    }
                },
                oninput: move |event| {
                    let input_text = event.value();
                    principal_input.set(input_text.clone());

                    // Allow empty input temporarily
                    if input_text.trim().is_empty() {
                        input_valid.set(true);
                        return;
                    }

                    // Remove commas and whitespace for parsing
                    let cleaned_input = input_text.replace(",", "").replace(" ", "");

                    match cleaned_input.parse::<f64>() {
                        Ok(value) if value > 0.0 && value.is_finite() => {
                            input_valid.set(true);
                            principal_signal.set(value);
                        }
                        Ok(value) if value <= 0.0 => {
                            input_valid.set(false);
                        }
                        Ok(_) => {
                            // Non-finite number (NaN, infinity)
                            input_valid.set(false);
                        }
                        Err(_) => {
                            input_valid.set(false);
                        }
                    }
                }
            }
            if !input_valid() && !principal_input().trim().is_empty() {
                div {
                    style: "color: #ff0000; font-size: 12px; margin-bottom: 10px;",
                    {
                        let input_text = principal_input();
                        let cleaned_input = input_text.replace(",", "").replace(" ", "");

                        if let Ok(value) = cleaned_input.parse::<f64>() {
                            if value <= 0.0 {
                                "Amount must be greater than zero"
                            } else {
                                "Invalid number format"
                            }
                        } else {
                            "Please enter a valid number (digits and decimal point only)"
                        }
                    }
                }
            }
        }

        div {
            Slider {
                class: "slider",
                label: "Interest Rate Slider",
                horizontal: true,
                min: 0.0,
                max: 50.0,
                step: 0.01,
                default_value: SliderValue::Single(0.05),
                on_value_change: move |value: SliderValue| {
                    let SliderValue::Single(v) = value;
                    current_value.set(v);
                },
                SliderTrack { class: "slider-track",
                    SliderRange { class: "slider-range" }
                    SliderThumb { class: "slider-thumb" }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
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
        Echo {}
        MySlider {}
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
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
