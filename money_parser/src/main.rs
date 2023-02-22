use std::{env, process};

#[derive(Debug)]
enum ParseMoneyError {
    InvalidInput,
    ParseFailed,
}

fn main() {
    let Some(user_input) = env::args().nth(1) else {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        return;
    };

    match parse_money(&user_input) {
        Ok((amount, currency)) => println!("Amount: {}, currency: {}", amount, currency),
        Err(error) => {
            eprintln!("Parsing error: {:?}", error);
            process::exit(1);
        }
    }
}

fn parse_money(input: &str) -> Result<(f32, &str), ParseMoneyError> {
    // Improve the split mechanism using a dedicated function that splits on whitespace.
    let segments: Vec<&str> = input.split_whitespace().collect();

    // Evaluate a "slice" of the segments...
    match segments[..] {
        // if it has at least 2 elements...
        [amount, currency] => {
            amount
                // the parse to a f32...
                .parse::<f32>()
                // if the result is the `Ok` variant, return a tuple with amount and the currency...
                .map(|amount| (amount, currency))
                // but if the result is the `Err` variant, return an error.
                .map_err(|_| ParseMoneyError::ParseFailed)
        }

        // This arm is evaluated if the match could not get the 2 segments. Let's return early and notify the caller of the error.
        _ => Err(ParseMoneyError::InvalidInput),
    }
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
