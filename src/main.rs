pub mod receipt;
use crate::receipt::content::*;
use crate::receipt::product::*;
use std::io::*;
use std::io;
use std::fs::File;

fn main() {
    let products = create_products();
    let mut cart = ReceiptContent {
    products: Vec::new(),
    store: String::from("Imaginary Town General Store"),
    };
    loop {
        println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim();
        match choice {
            "1" => {
                println!("Which product would you like to add?");
                println!("1) Zbox 720 | Price - 600");
                println!("2) GPU - AND Random RT6600 | Price - 200");
                println!("3) Potato | Price - 1");
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Error reading input.");
                let choice = choice.trim();
                match choice {
                    "1" => add_to_cart(&mut cart, &products, 0),
                    "2" => add_to_cart(&mut cart, &products, 1),
                    "3" => add_to_cart(&mut cart, &products, 2),
                    _ => {
                        println!("error reading input");
                    }
                }
            }
            "2" => {
                cart.products.pop();
            }
            "3" => {
                let _ = complete_purchase(&mut cart);
                println!("Thank you for your purchase!");
                break;
            }
            _ => {
                println!("wrong input");
            }
        }
    }
}