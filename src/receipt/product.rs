#[derive(Clone, PartialEq)]
pub struct StoreProduct {
    pub name: String,
    pub price: i32,
}

pub fn create_products() -> Vec<StoreProduct> {
    let product1 = StoreProduct {
        name: String::from("Zbox 720"),
        price: 600
    };
    let product2 = StoreProduct {
        name: String::from("GPU - AND Random RT6600"),
        price: 200
    };
    let product3 = StoreProduct {
        name: String::from("Potato"),
        price: 1
    };
    let products = vec![product1, product2, product3];
    return products;
}