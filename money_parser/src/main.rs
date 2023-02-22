use std::{env, process};

#[derive(Debug)]
enum ParseMoneyError {
    InvalidInput,
    ParseFailed,
}

fn main() {
    // env::args() returns an iterator, it's an useful type that allows manipulations or transformations on the inner
    // collection. We try to get the second argument which could not exists. This is why the next() function returns an
    // Option type. It has 2 variant: Some and None. Let's try to match against the Some variant, which would means we
    // got a value. If not, display the error message.
    let Some(user_input) = env::args().nth(1) else {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        return;
    };

    let result = parse_money(&user_input);

    if result.is_err() {
        eprintln!("Parsing error: {:?}", result.unwrap_err());
        process::exit(1);
    }

    let (amount, currency) = result.unwrap();

    println!("Amount: {}, currency: {}", amount, currency);
}

fn parse_money(input: &str) -> Result<(f32, &str), ParseMoneyError> {
    let segments: Vec<&str> = input.split(' ').collect();

    if segments.len() != 2 {
        return Err(ParseMoneyError::InvalidInput);
    }

    // A powerful construct is the "if let" and its opposite the "let else" control flow. Here, we try to match the
    // result of parsing to an i32 to the Ok variant. If it matches, the amount variable is created. If it doesn't,
    // the else scope is evaluated and we return an error.
    // As the result of `parse()` is immediately destructured we must help the `parse()` function to know which type it
    // should parse to using the "turbofish" operator.
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
