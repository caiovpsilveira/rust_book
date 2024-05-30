use supermarket::Supermarket;

mod supermarket;

fn main() {
    let mut supermarket = Supermarket::create();

    supermarket.insert_product("Milk", 10.0);
    supermarket.insert_product("Coffe", 15.0);

    // let products = supermarket.products; // does not compile, is private

    match supermarket.get_product("Milk") {
        Some(p) => println!("Found product. Price: {}", p.price),
        None => println!("Product not found."),
    }

    match supermarket.get_product("Coffe") {
        Some(p) => println!("Found product. Price: {}", p.price),
        None => println!("Product not found."),
    }

    match supermarket.get_product("Water") {
        Some(p) => println!("Found product. Price: {}", p.price),
        None => println!("Product not found."),
    }
}
