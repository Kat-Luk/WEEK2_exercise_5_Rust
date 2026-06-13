#[derive(Clone)]
pub struct StoreProduct {
    pub name: String,
    pub price: i32,
}

pub fn create_products() -> [StoreProduct; 3]{
    let product1 = StoreProduct {
        name: String::from("Zbox 720"),
        price: 600
    };
    let product2 = StoreProduct {
        name: String::from("GPU - AND Random RT6600"),
        price: 200
    };
    let product3 = StoreProduct {
        name: String::from("potato"),
        price: 1
    };
    let products = [product1, product2, product3];
    return products;
}