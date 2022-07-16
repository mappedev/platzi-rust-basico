fn main() {
    let name: &str = "Mario"; // * Variable inmutable
    let mut age: u8 = 24; // * Variable mutable

    age = age + 1;

    println!("Hi, I'm {} and I'm {} years old!", name, age);

    // * RETO
    let ccs_min_temp: i8 = 17;
    let ccs_max_temp: i8 = 28;

    println!("La máxima temperatura de caracas es: {}°C", ccs_max_temp);
    println!("La mínima temperatura de caracas es: {}°C", ccs_min_temp);
}
