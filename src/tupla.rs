fn main() {
    let tuple = (1, 'H', "Hola", false);
    let (i, c, s, b) = tuple;

    println!("Int: {}", i);
    println!("Char: {}", c);
    println!("String: {}", s);
    println!("Boolean: {}", b);

    // Métodos
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
    println!("{}", tuple.3);
}
