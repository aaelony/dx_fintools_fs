use num::Float;
use num_format::{Locale, ToFormattedString};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Compounding {
    Annual,
    Semiannually,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
    Other(f64),
}

impl Compounding {
    pub fn periods_per_year(&self) -> f64 {
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

    pub fn to_string(&self) -> &'static str {
        match self {
            Compounding::Annual => "Annually",
            Compounding::Semiannually => "Semi-annually",
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
