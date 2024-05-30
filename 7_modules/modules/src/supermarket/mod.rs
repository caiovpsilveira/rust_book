pub struct Product {
    pub name: String,
    pub price: f64,
}

pub struct Supermarket {
    products: Vec<Product>,
    id_count: u32,
}

impl Supermarket {
    pub fn create() -> Supermarket {
        Supermarket {
            products: Vec::new(),
            id_count: 0,
        }
    }

    pub fn insert_product(&mut self, name: &str, price: f64) {
        self.products.push(Product {
            name: String::from(name),
            price,
        });
        self.id_count += 1
    }

    pub fn get_product(&self, name: &str) -> Option<&Product> {
        for prod in self.products.iter() {
            if prod.name == name {
                return Some(prod);
            }
        }
        None
    }
}
