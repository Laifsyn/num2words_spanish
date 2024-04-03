use std::io::Write;

use num2words::lang::{Language, Spanish};
use num_bigfloat::BigFloat;
pub fn main() {
    let es = Spanish::default();
    let result = es.to_ordinal(BigFloat::from(1215));
    println!("{:?}", result);
    // let mut input = String::new();
    // print!("\nIngrese un número para convertir a palabras\nIngrese `exit` para salir:\n\n");
    // fn read_line(input: &mut String) {
    //     input.clear();
    //     std::io::stdin().read_line(input).unwrap();
    // }
    // loop {
    //     print!("Ingrese su número: ");
    //     flush();
    //     read_line(&mut input);
    //     let input = input.trim();
    //     match input {
    //         "exit" => {
    //             clear_terminal();
    //             println!("Saliendo...");
    //             break;
    //         }
    //         "clear" => {
    //             clear_terminal();
    //             continue;
    //         }
    //         _ => {}
    //     }
    //     if input.is_empty() {
    //         println!("Número inválido {input:?} no puede estar vacío");
    //         continue;
    //     }
    //     let num = match input.parse::<i128>() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Número inválido {input:?} - no es convertible a un número entero");
    //             continue;
    //         }
    //     };
    //     print!("Entrada:");
    //     pretty_print_int(num);
    //     println!(" => {:?}", es.to_int_cardinal(num.into()).unwrap());
    // }
}
pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
pub fn back_space(amount: usize) {
    for _i in 0..amount {
        print!("{}", 8u8 as char);
    }
    flush();
}
pub fn flush() {
    std::io::stdout().flush().unwrap();
}
pub fn pretty_print_int<T: Into<i128>>(num: T) {
    let mut num: i128 = num.into();
    let mut vec = vec![];
    while num > 0 {
        vec.push((num % 1000) as i16);
        num /= 1000;
    }
    vec.reverse();
    let prettied =
        vec.into_iter().map(|num| format!("{num:03}")).collect::<Vec<String>>().join(",");

    print!("{:?}", prettied.trim_start_matches('0'));
    flush();
}
