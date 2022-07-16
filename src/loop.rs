fn main() {
    let number_1 = 123;
    let number_2 = 321;

    let sum = number_1 + number_2;

    loop {
        println!("Por favor escribir la suma de {} y {}", number_1, number_2);

        let mut sum_input = String::new();
        std::io::stdin().read_line(&mut sum_input).unwrap();
        let sum_user: i32 = sum_input.trim().parse().unwrap();

        if sum_user == sum {
            println!("Lo has hecho bien! El resultado {} es correcto", sum_user);
            break;
        } else {
            println!(
                "El resultado {} es incorrecto, vuelve a intentar.",
                sum_user
            );
        }
    }
}
