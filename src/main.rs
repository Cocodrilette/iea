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

    match args {
        ParseResult::Values(values) => {
            let i_p = get_i_p(values.period, values.i_rate);
            let pmt = get_pmt(values.amount, i_p, values.period);
            let total = get_total_cost(pmt, values.period);

            print_result(values.amount, total, pmt);
        }
        ParseResult::Error(_) => help(),
    }
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
                return ParseResult::Error(true);
            }

            parsed_args.amount = value;
        }

        if kv.key == INTERES_RATE_OPTION {
            let value = string_to_float(kv.value.clone());
            if value == 0.0 {
                return ParseResult::Error(true);
            }

            parsed_args.i_rate = value;
        }

        if kv.key == PERIOD_OPTION {
            let value = string_to_float(kv.value.clone());
            if value == 0.0 {
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

fn string_to_float(string_value: String) -> f64 {
    match string_value.to_string().parse::<f64>() {
        Ok(value) => value,
        Err(_) => 0.0,
    }
}

fn get_i_p(p: f64, i: f64) -> f64 {
    return (1.0 + (i / 100.0)).powf(1.0 / p) - 1.0;
}

fn get_pmt(a: f64, i_p: f64, p: f64) -> f64 {
    return (a * i_p) / (1.0 - (1.0 + i_p).powf(-p));
}

fn get_total_cost(pmt: f64, p: f64) -> f64 {
    return pmt * p;
}

fn print_result(inicial: f64, total: f64, pmt: f64) {
    println!(
        r#"
Total: {}
Costo/Ganancia: {}
Pagos periodicos: {}
            "#,
        total,
        total - inicial,
        pmt
    );
}

fn help() {
    // value of literal (truncated up to newline):
    let help_message: &str = r#"
Usage: iea [OPTIONS]

Options:
    -p The number of payments pear year 
    -a The amount of money to calculate the interest over
    -i The interest rate percentage. For example: 12.5

Example:

    iea -p day -a 1000000 -i 13
"#;

    println!("{}", help_message);
}

// ****************************************************************
// **** Tests
// ****************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    static A: f64 = 10000.0; // Monto del prestamos
    static P: f64 = 12.0; // 12 pagos anuales
    static I: f64 = 12.0; // 12% de interes efectivo anual

    #[test]
    fn test_get_i_p() {
        let result = get_i_p(P, I); // tasa de interes periodica
        assert_eq!(result, 0.009488792934583046);
    }

    #[test]
    fn test_get_pmt() {
        let i_p = get_i_p(P, I);
        let result = get_pmt(A, i_p, P); // pago periodico
        assert_eq!(result, 885.6206738944115);
    }
}
