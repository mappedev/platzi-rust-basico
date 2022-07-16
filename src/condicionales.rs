fn main() {
    // println!("Introduce tu edad:");
    // let mut age: String = String::new();
    // std::io::stdin().read_line(&mut age).unwrap();

    // let age_int: u8 = age.trim().parse().unwrap();

    // if age_int >= 18 && age_int != 25 {
    //     println!("Eres mayor de edad.");

    //     if age_int > 70 {
    //         println!("¡Tienes bastante experiencia!");
    //     }
    // } else if age_int == 25 {
    //     println!("Tienes la misma edad que Mario.")
    // } else {
    //     println!("Eres menor de edad.")
    // }

    // println!("Tienes {} años de edad", age_int);

    // // * Podemos usar un condicional al momento de asignar un valor en una variable
    // let variable = if true { "Se cumplio" } else { "No se cumplió" };
    // println!("{}", variable);

    // * RETO
    println!("Selecciona la pildora roja o azul:");
    let mut selection = String::new();
    std::io::stdin().read_line(&mut selection).unwrap();

    selection = selection.trim().to_string();

    if selection == "roja" {
        println!("Vuelve a la realidad.");
    } else if selection == "azul" {
        println!("Vamos a la Matrix.");
    } else {
        println!("Hasta luego...")
    }
}
