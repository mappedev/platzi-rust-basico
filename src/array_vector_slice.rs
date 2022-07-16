fn main() {
    // * VECTOR
    // * El vector es mutable, tanto su tamaño cómo sus elementos.
    let mut fullname_split: Vec<String> = Vec::new();

    println!("Introduce la cantidad de palabras que tiene tu nombre completo:");
    let mut quantity_input = String::new();
    std::io::stdin().read_line(&mut quantity_input).unwrap();
    let quantity: i32 = quantity_input.trim().parse().unwrap();

    for i in 0..quantity {
        println!("Por favor introduce tu nombre #{}:", i + 1);
        let mut name_input = String::new();
        std::io::stdin().read_line(&mut name_input).unwrap();
        let name = name_input.trim().to_string();

        fullname_split.push(name);
    }

    println!("{:?}", fullname_split);

    for (i, name) in fullname_split.iter().enumerate() {
        println!("Nombre #{}: {}", i + 1, name);
    }

    // * Methods
    println!("{}", fullname_split.len());

    // * Con [i] podemos obtener el valor, en caso de que ocurra un error, no lo podemos controlar.
    // println!("Primer nombre: {}", fullname_split[0]);
    // * RECOMENDADA! Con get podemos obtener el valor y el nos devuelve un Option, así podemos saber si el valor existe o no.
    println!("Primer nombre: {}", fullname_split.get(0).unwrap());

    // * ARRAY
    // * El array es inmutable, tanto su tamaño cómo sus elementos.

    let hola_chars = ['H', 'o', 'l', 'a'];
    let numbers = [1, 2, 3];
    let a = [0; 3]; // * Con esto creamos un array que tendra todos los valores en 0, 3 veces [v;q]
    println!("{:?}", hola_chars);
    println!("{:?}", a);

    for c in hola_chars {
        println!("{}", c);
    }

    for i in numbers {
        println!("{}", i);
    }

    // * Methods
    // * Con el array tambiém podemos obtener los valores de la misma manera que con los vectores
    println!("Primer caracter de la palabra hola: {}", hola_chars[0]);
    println!(
        "Primer caracter de la palabra hola: {}",
        hola_chars.get(0).unwrap()
    );

    // * SLICE
    // * Un slice es un fragmento o pedazo de un array o vector.
    println!("{:?}", &fullname_split[2..]);
    println!("{:?}", &hola_chars[..2]);

    // Extend
    // * Extender un vector con un slice (array)
    let mut union: Vec<char> = Vec::new();
    union.extend_from_slice(&hola_chars);
    println!("{:?}", union);
}
