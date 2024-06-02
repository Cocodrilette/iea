use std::env;

struct KeyValueArg {
    key: String,
    value: String,
}

#[derive(Debug)]
struct IEAValues {
    period: f64,
    amount: f64,
    i_rate: f64,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParseResult {
    Values(IEAValues),
    Error(bool),
}

static AMOUNT_OPTIONS: &str = "-a";
static INTERES_RATE_OPTION: &str = "-i";
static PERIOD_OPTION: &str = "-p";

fn main() {
    let args = parse_command_line_args();
    println!("{:?}", args)
}

fn parse_command_line_args() -> ParseResult {
    /*
     * - `::` is used for accessing associated functions, which are essentially static methods in Rust.
     * They are defined within an impl block but don't take a self parameter.
     * They are called on the type itself, not on an instance of the type.
     * For example, String::from("hello") is calling the associated function from on the String type.
     *
     * - . is used for accessing methods on an instance of a type. These methods take a self, &self, or
     * &mut self parameter, which refers to the instance of the type that the method is being called on.
     * For example, in "hello".to_string(), to_string is a method being called on the instance "hello".
     */
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // Remove the fist argument because it is the current path

    if args.len() % 2 != 0 {
        help();
        return ParseResult::Error(true);
    }

    let kv_pairs = get_kv(args);

    let mut parsed_args = IEAValues {
        amount: 0.0,
        i_rate: 0.0,
        period: 0.0,
    };
    for kv in kv_pairs {
        /*
         * In Rust, values are owned by a single variable, and when they are assigned
         * to another variable or passed to a function, they are moved, not copied
         * (unless the type implements the Copy trait). After a value has been moved,
         * it can no longer be used from the original variable. Thats the reason for the
         * `.clone()` here
         */
        if kv.key == AMOUNT_OPTIONS {
            let value = string_to_float(kv.value.clone());
            if value == 0.0 {
                help();
                return ParseResult::Error(true);
            }

            parsed_args.amount = value;
        }

        if kv.key == INTERES_RATE_OPTION {
            let value = string_to_float(kv.value.clone());
            if value == 0.0 {
                help();
                return ParseResult::Error(true);
            }

            parsed_args.i_rate = value;
        }

        if kv.key == PERIOD_OPTION {
            let value = string_to_float(kv.value.clone());
            if value == 0.0 {
                help();
                return ParseResult::Error(true);
            }

            parsed_args.period = value;
        }
    }

    return ParseResult::Values(parsed_args);
}

fn get_kv(args: Vec<String>) -> Vec<KeyValueArg> {
    let mut kv_vec: Vec<KeyValueArg> = Vec::new();
    for i in (0..args.len()).step_by(2) {
        kv_vec.push(KeyValueArg {
            key: args[i].clone(),
            value: args[i + 1].clone(),
        })
    }

    return kv_vec;
}

fn help() {
    // value of literal (truncated up to newline):
    let help_message: &str = r#"
Usage: iea [OPTIONS]

Options:
    -p The payment period. Supported values are one of those: day, month, year 
    -a The amount of money to calculate the interest over
    -i The interest rate percentage. For example: 12.5

Example:

    iea -p day -a 1000000 -i 13
"#;

    println!("{}", help_message);
}

fn string_to_float(string_value: String) -> f64 {
    match string_value.to_string().parse() {
        Ok(value) => value,
        Err(_) => 0.0,
    }
}
