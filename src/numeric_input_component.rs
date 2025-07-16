use dioxus::prelude::*;

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

#[component]
pub fn NumericInput(
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
                placeholder,
                value: "{input_signal}",
                class: if valid_signal() { format!("{}-input-valid", css_prefix) } else { format!("{}-input-invalid", css_prefix) },
                style: {
                    let input_width = std::cmp::max(100, input_signal().len() * 9 + 20);
                    format!(
                        "padding: 6px 8px; width: {}px; border-radius: 4px; font-family: monospace;",
                        input_width,
                    )
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
