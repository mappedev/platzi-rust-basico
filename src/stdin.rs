fn main() {
    // println!("Por favor introduce tu nombre:");
    // let mut name: String = String::new();
    // std::io::stdin().read_line(&mut name).unwrap();
    // name = name.trim().to_string();

    // println!("Por favor introduce tu edad:");
    // let mut age: String = String::new();
    // std::io::stdin().read_line(&mut age).unwrap();
    // let age_int: u8 = age.trim().parse().unwrap(); // * Convertir string a int

    // println!("Bienvenid@ {}, tienes {} años.", name, age_int);

    // * RETO
    let mut name = String::new();
    let mut nation = String::new();

    println!("Nombre:");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Nación:");
    std::io::stdin().read_line(&mut nation).unwrap();

    name = name.trim().to_string();
    nation = nation.trim().to_string();
    println!("Tu nombre es {} y tu nación es {}", name, nation);
}
