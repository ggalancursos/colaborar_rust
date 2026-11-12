use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    precio_billete();
    pension();
}

fn pide_dato<T: std::str::FromStr>(texto: &str, error: &str, validador: Option<fn(&T) -> bool>) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut entrada: String = String::new();
    print!("{}", texto);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la linea");

    let mut valor;

    if let Some(primer_caracter) = entrada.trim().chars().next() {
        if primer_caracter.is_numeric() {
            valor = entrada.trim().replace(",", ".").parse::<T>().expect(error);
        } else {
            valor = entrada.trim().parse::<T>().expect(error);
        }
    } else {
        panic!("{}", error);
    }

    if let Some(validar) = validador {
        if !validar(&valor) {
            panic!("{}", error);
        }
    }

    valor
}

fn precio_billete() {
    const PRICE_KM: f64 = 0.3;
    const MIN_KM_DIS: f64 = 80.0;
    const DISCOUNT_BASE: f64 = 0.15;
    const DISCOUNT_EXTRA: f64 = 0.05;

    let distance: f64 = pide_dato("introduce distancia de trayecto ","distancia erronea",None);
    let day_type: String = pide_dato("¿qué tipo de día es el día del trayecto? (L/F) ","tipo de día erroneo",None);
    let raw_price = distance * PRICE_KM;

    let mut net_price = 0.0;
    let mut discount = 0.0;

    if distance > MIN_KM_DIS {discount += DISCOUNT_BASE;}
    if day_type == "L" {discount += DISCOUNT_EXTRA;}

    net_price = raw_price - (raw_price * discount);

    println!("Precio base: {}", raw_price);
    println!("descuento aplicado: {}", discount);
    println!(" El precio final del billete es: {}", net_price);
}

fn pension() {
    const PENSION_MAX: f64 = 2300.0;
    const PORCENTAJE: f64 = 0.8;
    const NUM_PAGAS: f64 = 14.0;

    let salario_bruto:f64 = pide_dato("Introduce tu salario bruto anual: ", "dato erroneo", None);
    let mut pension = salario_bruto / NUM_PAGAS * PORCENTAJE;

    if pension > PENSION_MAX {pension = PENSION_MAX;}
    println!("Tu pensión es: {:.2}", pension);
}
