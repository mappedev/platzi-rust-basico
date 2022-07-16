fn main() {
    let mut dictionary: HashMap<&str, &str> = HashMap::new();
    dictionary.insert("manzana", "Fruta de color rojo o verde");
    dictionary.insert("pera", "Fruta de color verde");

    println!("Manzana: {:?}", dictionary["manzana"]);
}
