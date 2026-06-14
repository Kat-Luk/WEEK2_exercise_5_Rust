use crate::receipt::product::*;
use crate::receipt::product;
use crate::File;
use std::io::Write;
const PRODUCT_1_NAME: &str = "Zbox 720";
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
const PRODUCT_3_NAME: &str = "Potato";

pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}

pub fn add_to_cart(cart: &mut ReceiptContent, products: &Vec<product::StoreProduct>, choice: usize) {
    cart.products.push(products[choice].clone());
}

pub fn complete_purchase(cart: &mut ReceiptContent) -> Result<(), String>{
    let mut total = 0;
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    let mut total_p3 = 0;
    let mut price_p1 = 0;
    let mut price_p2 = 0;
    let mut price_p3 = 0;
    for product in &cart.products {
        total += product.price;
        if product.name == "Zbox 720" {
            total_p1 += 1;
            price_p1 = product.price;
        } else if product.name == "GPU - AND Random RT6600" {
            total_p2 += 1;
            price_p2 = product.price;
        } else {
            total_p3 += 1;
            price_p3 = product.price;
        }
    }
    let mut file = File::create("receipt.txt").map_err(|e| e.to_string())?;
    writeln!(file, "{}", cart.store).expect("Error writing!");
    writeln!(file, "------------------------------").expect("Error writing!");
    if total_p1 > 0 {
        writeln!(file, "Zbox 720 ({}) - {}€", total_p1, price_p1).expect("Error writing!");
    }
    if total_p2 > 0 {
        writeln!(file, "GPU - AND Random RT6600 ({}) - {}€", total_p2, price_p2).expect("Error writing!");
    }
    if total_p3 > 0 {
        writeln!(file, "Potato ({}) - {}€", total_p3, price_p3).expect("Error writing!");
    }
    writeln!(file, "------------------------------").expect("Error writing!");
    writeln!(file, "Final price: {}€", total).expect("Error writing!");
    writeln!(file, "------------------------------").expect("Error writing!");    
    return Ok(());
}