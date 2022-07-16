// fn get_ownership(s: String) {
//     println!("{}", s)
// }

// fn change(s: &mut String) {
//     s.push_str(", World!")
// }

// ! Esto puede generará una referencia colgante
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// * Solución a las referencias colgantes
// fn dangle() -> String {
//     let s = String::from("hello");
//     s
// }

fn first_word(words: &str) -> &str {
    let words: Vec<&str> = words.split(' ').collect();
    words.first().unwrap()
}

fn first_word_imperative(words: &str) -> &str {
    for (i, &c) in words.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &words[..i];
        }
    }

    words
}

fn main() {
    // * PROPIEDAD
    /*
    Se pueden copiar los valores si lo datos son de tipo:
        * Enteros, como u32.
        * Booleano, como true o false.
        * float, como f64.
        * Carácter.
        * Tuplas o listas que contengan los indicados anteriormente.
    */
    // let a = "hola";
    // let b = a;
    // println!("{} {}", a, b); // * No problem

    // get_ownership(a.to_string());
    // println!("{}", a); // * No problem

    // ! Con las variables que no se conoce el valor al momento de compilar no se le copia el valor, se pasa la propiedad del valor
    // let c = String::from("Hola");
    // let d = c; // ! Err
    // println!("{} {}", c, d);

    // let msg = String::from("Message");
    // get_ownership(msg); // Hasta luego propiedad
    // println!("{}", msg) // ! Err

    // * REFERENCIAS Y PRÉSTAMO
    // let mut s = String::from("Hello");

    // println!("{}", s);
    // change(&mut s); // Pasamos la variable mutable para que se pueda cambiar su valor
    // println!("{}", s);

    /*
    Las referencias mutables tienen una gran restricción:
    Si tiene una referencia mutable a un valor, no puede
    tener otras referencias a ese valor.
    */
    // let mut s2 = String::from("Hello");
    // let r1 = &mut s2;
    // let r2 = &mut s2; // ! Err solo se puede tener una referencia mutable a un valor
    // println!("{} {}", r1, r2)

    // let mut s3 = String::from("Hello");
    // let r1 = &s3;
    // let r2 = &s3;
    // let r3 = &mut s3;

    // println!("{} {} {}", r1, r2, r3); // ! Err No se pueden realizar referencias mutables si del mismo valor ya se tiene una referencia inmutable

    // * Forma de solventar el error anterior:
    // let mut s3 = String::from("Hello");
    // let r1 = &s3;
    // let r2 = &s3;
    // println!("{} {}", r1, r2); // * Último momento en que las variable referenciada inmutables de usó

    // let r3 = &mut s3;
    // println!("{}", r3);
    // * Es posible por ya el ciclo de vida de las variables r1 y r2 se consumió así esté en el mismo scope
    // * A esto se le conoce como Non-Lexical Lifetimes (NLL para abreviar).

    // REFERENCIAS COLGANTES
    // let reference_to_nothing = dangle(); // ! Err el valor referenciado que devuelve la fn dangle murere al dejar terminar su contexto de uso
    // println!("{}", reference_to_nothing);

    // * Solución al problema anterior (revisa la función dangle):
    // let s4 = dangle();
    // println!("{}", s4);

    // SLICES
    // String
    let words_str = "hola compa";
    let first_word = first_word(&words_str);
    println!("{}", first_word);

    let first_word = first_word_imperative(&words_str);
    println!("{}", first_word);

    // Array y Vector
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", &array[1..4]);

    let vector = Vec::from([1, 2, 3, 4, 5]);
    println!("{:?}", &vector[1..4]);
}
