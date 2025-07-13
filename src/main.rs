use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};
use num::Float;
use num_format::{Locale, ToFormattedString};
use tracing::debug;
use tracing_subscriber;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Compounding {
    Annual,
    Semiannually,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
    Other(f64),
}

impl Compounding {
    fn periods_per_year(&self) -> f64 {
        match self {
            Compounding::Annual => 1.0,
            Compounding::Semiannually => 2.0,
            Compounding::Quarterly => 4.0,
            Compounding::Monthly => 12.0,
            Compounding::Weekly => 52.0,
            Compounding::Daily => 365.0,
            Compounding::Other(periods) => *periods,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Compounding::Annual => "Annual",
            Compounding::Semiannually => "Semiannual",
            Compounding::Quarterly => "Quarterly",
            Compounding::Monthly => "Monthly",
            Compounding::Weekly => "Weekly",
            Compounding::Daily => "Daily",
            Compounding::Other(_) => "Custom",
        }
    }
}

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
    let mut years_signal = use_signal(|| 1.0);
    let mut periods_per_year_signal = use_signal(|| Compounding::Annual);
    let mut principal_signal = use_signal(|| 1000.00 as f64);
    let mut input_valid = use_signal(|| true);
    let mut years_input = use_signal(|| "1.0".to_string());
    let mut years_input_valid = use_signal(|| true);
    // let principal_amount = 10_000.0f64;
    let principal_amount = principal_signal();
    let years = years_signal();
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
                    match value.as_str() {
                        "annual" => periods_per_year_signal.set(Compounding::Annual),
                        "semiannual" => periods_per_year_signal.set(Compounding::Semiannually),
                        "quarterly" => periods_per_year_signal.set(Compounding::Quarterly),
                        "monthly" => periods_per_year_signal.set(Compounding::Monthly),
                        "weekly" => periods_per_year_signal.set(Compounding::Weekly),
                        "daily" => periods_per_year_signal.set(Compounding::Daily),
                        _ => {}
                    }
                },
                option {
                    value: "annual",
                    selected: matches!(periods_per_year_signal(), Compounding::Annual),
                    style: "background: gray; color: white;",
                    "Annual"
                }
                option {
                    value: "semiannual",
                    selected: matches!(periods_per_year_signal(), Compounding::Semiannually),
                    "Semi-annually"
                }
                option {
                    value: "quarterly",
                    selected: matches!(periods_per_year_signal(), Compounding::Quarterly),
                    "Quarterly"
                }
                option {
                    value: "monthly",
                    selected: matches!(periods_per_year_signal(), Compounding::Monthly),
                    "Monthly"
                }
                option {
                    value: "weekly",
                    selected: matches!(periods_per_year_signal(), Compounding::Weekly),
                    "Weekly"
                }
                option {
                    value: "daily",
                    selected: matches!(periods_per_year_signal(), Compounding::Daily),
                    "Daily"
                }
            }
        }
        //
        // Input Principal
        div { style: "display: flex; align-items: center; margin-bottom: 15px;",
            label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
                "Principal Amount ($):"
            }
            input {
                placeholder: "Enter initial principal amount (e.g., 10000.00)",
                value: "{principal_signal}",
                style: {
                    let input_width = std::cmp::max(
                        100,
                        format!("{}", principal_signal()).len() * 9 + 20,
                    );
                    if input_valid() {
                        format!(
                            "border: 1px solid #ccc; padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;",
                            input_width,
                        )
                    } else {
                        format!(
                            "border: 2px solid #ff0000; padding: 6px 8px; background-color: #ffe6e6; width: {}px; border-radius: 4px; font-family: monospace;",
                            input_width,
                        )
                    }
                },
                oninput: move |event| {
                    let input_text = event.value();
                    let cleaned_input = input_text.replace(",", "").replace(" ", "");
                    if input_text.trim().is_empty() {
                        input_valid.set(true);
                        return;
                    }
                    match cleaned_input.parse::<f64>() {
                        Ok(value) if value > 0.0 && value.is_finite() => {
                            input_valid.set(true);
                            principal_signal.set(value);
                        }
                        Ok(value) if value <= 0.0 => {
                            input_valid.set(false);
                        }
                        Ok(_) => {
                            input_valid.set(false);
                        }
                        Err(_) => {
                            input_valid.set(false);
                        }
                    }
                },
            }
        }
        if !input_valid() {
            div { style: "color: #ff0000; font-size: 12px; margin-left: 160px; margin-bottom: 10px;",
                "Please enter a valid positive number"
            }
        }

        // Input Years
        div { style: "display: flex; align-items: center; margin-bottom: 15px;",
            label { style: "margin-right: 10px; font-weight: bold; color: #333; min-width: 150px;",
                "Number of Years:"
            }
            input {
                placeholder: "Enter number of years (e.g., 5.0)",
                value: "{years_input}",
                style: {
                    let input_width = std::cmp::max(80, years_input().len() * 9 + 20);
                    if years_input_valid() {
                        format!(
                            "border: 1px solid #ccc; padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;",
                            input_width,
                        )
                    } else {
                        format!(
                            "border: 2px solid #ff0000; padding: 6px 8px; background-color: #ffe6e6; width: {}px; border-radius: 4px; font-family: monospace;",
                            input_width,
                        )
                    }
                },
                oninput: move |event| {
                    let input_text = event.value();
                    years_input.set(input_text.clone());
                    if input_text.trim().is_empty() {
                        years_input_valid.set(true);
                        return;
                    }
                    let cleaned_input = input_text.replace(",", "").replace(" ", "");
                    match cleaned_input.parse::<f64>() {
                        Ok(value) if value > 0.0 && value.is_finite() => {
                            years_input_valid.set(true);
                            years_signal.set(value);
                        }
                        Ok(value) if value <= 0.0 => {
                            years_input_valid.set(false);
                        }
                        Ok(_) => {
                            years_input_valid.set(false);
                        }
                        Err(_) => {
                            years_input_valid.set(false);
                        }
                    }
                },
            }
        }
        if !years_input_valid() && !years_input().trim().is_empty() {
            div { style: "color: #ff0000; font-size: 12px; margin-left: 160px; margin-bottom: 10px;",
                {
                    let input_text = years_input();
                    let cleaned_input = input_text.replace(",", "").replace(" ", "");
                    if let Ok(value) = cleaned_input.parse::<f64>() {
                        if value <= 0.0 {
                            "Number of years must be greater than zero"
                        } else {
                            "Invalid number format"
                        }
                    } else {
                        "Please enter a valid number (digits and decimal point only)"
                    }
                }
            }
        }

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
