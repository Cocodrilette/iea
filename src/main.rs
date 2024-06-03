use std::env;
use std::process;

enum Period {
    Year,
    Semester,
    Month,
    Week,
    Day,
}

impl Period {
    fn from_str(period: &str) -> Option<Period> {
        match period.to_lowercase().as_str() {
            "year" | "año" => Some(Period::Year),
            "semester" | "semestre" => Some(Period::Semester),
            "month" | "mes" => Some(Period::Month),
            "week" | "semana" => Some(Period::Week),
            "day" | "día" => Some(Period::Day),
            _ => None,
        }
    }

    fn periods_per_year(&self) -> f64 {
        match self {
            Period::Year => 1.0,
            Period::Semester => 2.0,
            Period::Month => 12.0,
            Period::Week => 52.0,
            Period::Day => 365.0,
        }
    }
}

fn main() {
    // Leer los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Uso: {} <capital_inicial> <interes_efectivo_anual> <periodo_de_pago>",
            args[0]
        );
        process::exit(1);
    }

    let capital_inicial: f64 = args[1]
        .parse::<f64>()
        .expect("El capital inicial debe ser un número.");
    let interes_efectivo_anual: f64 = args[2]
        .parse::<f64>()
        .expect("El interés efectivo anual debe ser un número.")
        / 100.0; // Convertir porcentaje a decimal
    let periodo_de_pago = Period::from_str(&args[3])
        .expect("El periodo de pago debe ser uno de: año, semestre, mes, semana, día.");

    // Número de periodos por año
    let periodos_por_año = periodo_de_pago.periods_per_year();

    // Calcular la tasa de interés por periodo
    let tasa_periodica = (1.0 + interes_efectivo_anual).powf(1.0 / periodos_por_año) - 1.0;

    // Número total de periodos (usamos 1 año para simplicidad)
    // @todo permitir ingresar el numero de años
    let num_periodos = 1.0 * periodos_por_año;

    // Calcular el valor final y las ganancias/costos totales
    let valor_final = capital_inicial * (1.0 + tasa_periodica).powf(num_periodos);
    let ganancias_o_costos = valor_final - capital_inicial;

    // Mostrar resultados
    println!("Tasa de interés periódica: {:.8}", tasa_periodica);
    println!("Valor final: {:.2}", valor_final);
    println!(
        "Ganancias/costos totales en un año: {:.2}",
        ganancias_o_costos
    );
}
