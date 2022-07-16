fn add1(num: i32) -> i32 {
    num + 1
}

fn main() {
    let mut counter_input = String::new();
    let mut quantity_input = String::new();

    println!("Ingrese el valor inicial del contador:");
    std::io::stdin().read_line(&mut counter_input).unwrap();
    println!("Ingrese la cantidad de veces que quiere aumentar el contador en 1:");
    std::io::stdin().read_line(&mut quantity_input).unwrap();

    let mut counter: i32 = counter_input.trim().parse().unwrap();
    let quantity: i32 = quantity_input.trim().parse().unwrap();

    for _i in 0..quantity {
        counter = add1(counter);
    }

    println!("Resultado: {}", counter);
}
