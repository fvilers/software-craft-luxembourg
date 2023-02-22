use std::{env, process};

#[derive(Debug)]
enum ParseMoneyError {
    InvalidInput,
    ParseFailed,
}

// A Money struct type, with a lifetime annotation (read bellow).
struct Money<'a> {
    amount: f32,
    currency: &'a str,
}

// The Money struct is implemented for a specific lifetime. This means that the struct must live long enough (as long as
// the currency parameter will live).
impl<'a> Money<'a> {
    fn new(amount: f32, currency: &'a str) -> Self {
        Self { amount, currency }
    }
}

fn main() {
    let Some(user_input) = env::args().nth(1) else {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        return;
    };

    match parse_money(&user_input) {
        Ok(money) => println!("Amount: {}, currency: {}", money.amount, money.currency),
        Err(error) => {
            eprintln!("Parsing error: {:?}", error);
            process::exit(1);
        }
    }
}

fn parse_money(input: &str) -> Result<Money, ParseMoneyError> {
    let segments: Vec<&str> = input.split_whitespace().collect();

    match segments[..] {
        [amount, currency] => amount
            .parse::<f32>()
            .map(|amount| Money::new(amount, currency))
            .map_err(|_| ParseMoneyError::ParseFailed),

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
        let money = parse_money("123 €").unwrap();

        assert_eq!(money.amount, 123.0);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn parse_money_should_return_float_amount_and_currency() {
        let money = parse_money("123.45 €").unwrap();

        assert_eq!(money.amount, 123.45);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_integer_amount_and_currency() {
        let money = parse_money("-123 €").unwrap();

        assert_eq!(money.amount, -123.0);
        assert_eq!(money.currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_float_amount_and_currency() {
        let money = parse_money("-123.45 €").unwrap();

        assert_eq!(money.amount, -123.45);
        assert_eq!(money.currency, "€");
    }
}
