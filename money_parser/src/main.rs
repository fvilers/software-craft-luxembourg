use std::{env, process};

// Create an enum to hold the various error that can be returned by the `parse_money()` function. The `derive` attribute
// is used by the compiler to automatically implement some basic functionality. The `Debug` trait is used to format
// strings with the `{:?}` placeholder.
#[derive(Debug)]
enum ParseMoneyError {
    InvalidInput,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        process::exit(1);
    }

    let result = parse_money(&args[1]);

    // The `Result` type has a lot of convenient function to work with. One of them is `is_err()` which allow us to
    // detect if an error occurred.
    if result.is_err() {
        // `unwrap_err()` returns the err variant.
        eprintln!("Parsing error: {:?}", result.unwrap_err());
        process::exit(1);
    }

    // The result variable is a `Result` type, `unwrap()` will extract the value but will "panic" if the result is `Err`
    // variant.
    let (amount, currency) = result.unwrap();

    println!("Amount: {}, currency: {}", amount, currency);
}

fn parse_money(input: &str) -> Result<(f32, &str), ParseMoneyError> {
    let segments: Vec<&str> = input.split(' ').collect();

    if segments.len() != 2 {
        // Return the `Err` variant of `Result` with the specific case of `InvalidInput`.
        return Err(ParseMoneyError::InvalidInput);
    }

    let amount: f32 = segments[0].parse().unwrap();

    // Return the `Ok` variant of `Result` with the usual tuple.
    Ok((amount, segments[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_money_should_return_invalid_input_const_for_empty_string() {
        let result = parse_money("");

        assert!(result.is_err());
    }

    #[test]
    fn parse_money_should_return_invalid_input_const_for_string_without_currency() {
        let result = parse_money("123");

        assert!(result.is_err());
    }

    #[test]
    fn parse_money_should_return_invalid_input_const_for_string_without_amount() {
        let result = parse_money("eur");

        assert!(result.is_err());
    }

    #[test]
    fn parse_money_should_return_integer_amount_and_currency() {
        let (amount, currency) = parse_money("123 €").unwrap();

        assert_eq!(amount, 123.0);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_float_amount_and_currency() {
        let (amount, currency) = parse_money("123.45 €").unwrap();

        assert_eq!(amount, 123.45);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_integer_amount_and_currency() {
        let (amount, currency) = parse_money("-123 €").unwrap();

        assert_eq!(amount, -123.0);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_float_amount_and_currency() {
        let (amount, currency) = parse_money("-123.45 €").unwrap();

        assert_eq!(amount, -123.45);
        assert_eq!(currency, "€");
    }
}
