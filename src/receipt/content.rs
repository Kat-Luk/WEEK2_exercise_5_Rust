use crate::receipt::product::*;
const PRODUCT_1_NAME: &str = "Zbox 720";
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
const PRODUCT_3_NAME: &str = "Potato";

pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}

pub fn add_to_cart(cart: &mut ReceiptContent, products: &[StoreProduct; 3], choice: usize) {
    cart.products.push(products[choice].clone());
}

pub fn complete_purchase(cart: &mut ReceiptContent) -> (i32, i32, i32, i32){
    let mut total = 0;
    let mut total_p1 =0;
    let mut total_p2 =0;
    let mut total_p3 =0;
    for product in &cart.products {
        total += product.price;
        if product.name == "Zbox 720" {
            total_p1 += 1;
        } else if product.name == "GPU - AND Random RT6600" {
            total_p2 += 1;
        } else {
            total_p3 += 1;
        }
    }
    return (total_p1, total_p2, total_p3, total);
}