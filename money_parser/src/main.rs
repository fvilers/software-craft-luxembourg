use std::{env, process};

// This is a magic number value indicating a parsing error. Don't do that at home, it's a temporary way of returning
// meaningful information from the `parse_money()` function.
const INVALID_INPUT: f32 = -1.0;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: money_parser.exe <value_to_parse>");
        process::exit(1);
    }

    // Call the `parse_money()` function with the second arguments (the first one is the executable path).
    let (amount, currency) = parse_money(&args[1]);

    // Display a message to the user if couldn't parse its input.
    if amount == INVALID_INPUT {
        eprintln!("Invalid input");
        process::exit(1);
    }

    // If we made it so far, everything is ok, let's print the amount and currency..
    println!("Amount: {}, currency: {}", amount, currency);
}

// The `parse_money()` function has one parameter which is a reference to a string (more on that later) and returns a
// tuple of a signed 32-bit float and a reference to a string.
fn parse_money(input: &str) -> (f32, &str) {
    // Split the input string on the white space character and collect the segments into a vector.
    let segments: Vec<&str> = input.split(' ').collect();

    // Again, let's fail early if the vector length is not exactly of two segments.
    if segments.len() != 2 {
        return (INVALID_INPUT, "");
    }

    // The compiler cannot infer to which type the value should be parsed so we annotate the amount variable with `f32`.
    // Don't pay attention to the `unwrap()` function yet.
    let amount: f32 = segments[0].parse().unwrap();

    // Look how convenient it is to omit the semi-colon and have the compiler use the expression as the return value!
    (amount, segments[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_money_should_return_invalid_input_const_for_empty_string() {
        let (amount, _) = parse_money("");

        assert_eq!(amount, INVALID_INPUT);
    }

    #[test]
    fn parse_money_should_return_invalid_input_const_for_string_without_currency() {
        let (amount, _) = parse_money("123");

        assert_eq!(amount, INVALID_INPUT);
    }

    #[test]
    fn parse_money_should_return_invalid_input_const_for_string_without_amount() {
        let (amount, _) = parse_money("eur");

        assert_eq!(amount, INVALID_INPUT);
    }

    #[test]
    fn parse_money_should_return_integer_amount_and_currency() {
        let (amount, currency) = parse_money("123 €");

        assert_eq!(amount, 123.0);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_float_amount_and_currency() {
        let (amount, currency) = parse_money("123.45 €");

        assert_eq!(amount, 123.45);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_integer_amount_and_currency() {
        let (amount, currency) = parse_money("-123 €");

        assert_eq!(amount, -123.0);
        assert_eq!(currency, "€");
    }

    #[test]
    fn parse_money_should_return_negative_float_amount_and_currency() {
        let (amount, currency) = parse_money("-123.45 €");

        assert_eq!(amount, -123.45);
        assert_eq!(currency, "€");
    }
}
