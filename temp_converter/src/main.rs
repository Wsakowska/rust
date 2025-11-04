use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Użycie: <program> <wartość> <jednostka_z> <jednostka_do>");
        eprintln!("Przykład: cargo run -- 36.6 C F");
        std::process::exit(1);
    }

    let value: f64 = args[1].parse().expect("Niepoprawna liczba!");
    let from = args[2].to_uppercase();
    let to = args[3].to_uppercase();

    let result = convert_temperature(value, &from, &to);

    match result {
        Some(converted) => println!("{value}°{from} = {converted:.2}°{to}"),
        None => eprintln!("Nieobsługiwana konwersja: {from} -> {to}"),
    }
}

fn convert_temperature(value: f64, from: &str, to: &str) -> Option<f64> {
    if from == to {
        return Some(value);
    }

    // Najpierw konwertujemy wszystko do Celsjusza
    let celsius = match from {
        "C" => value,
        "F" => (value - 32.0) * 5.0 / 9.0,
        "K" => value - 273.15,
        _ => return None,
    };

    // Następnie z Celsjusza na żądaną jednostkę
    let result = match to {
        "C" => celsius,
        "F" => celsius * 9.0 / 5.0 + 32.0,
        "K" => celsius + 273.15,
        _ => return None,
    };

    Some(result)
}
