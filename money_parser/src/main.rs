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

    // A match is a powerful control flow construct. It allows to match on different variant of a variable and execute
    // specific code for each of them. Moreover, the match construct will ask you to exhaust all of the variants in
    // order to let you compile the code.
    match parse_money(&user_input) {
        Ok((amount, currency)) => println!("Amount: {}, currency: {}", amount, currency),
        Err(error) => {
            eprintln!("Parsing error: {:?}", error);
            process::exit(1);
        }
    }
}

fn parse_money(input: &str) -> Result<(f32, &str), ParseMoneyError> {
    let segments: Vec<&str> = input.split(' ').collect();

    if segments.len() != 2 {
        return Err(ParseMoneyError::InvalidInput);
    }

    let Ok(amount) = segments[0].parse::<f32>() else {
        return Err(ParseMoneyError::ParseFailed);
    };

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
