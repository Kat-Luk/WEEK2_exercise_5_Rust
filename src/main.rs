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
                let (total_p1, total_p2, total_p3, total) = complete_purchase(&mut cart);
                let mut file = File::create("receipt.txt").expect("Error creating the file!");
                writeln!(file, "Imaginary Town General Store").expect("Error writing!");
                writeln!(file, "------------------------------").expect("Error writing!");
                if total_p1 > 0 {
                    writeln!(file, "Zbox 720 ({}) - 600€", total_p1).expect("Error writing!");
                }
                if total_p2 > 0 {
                    writeln!(file, "GPU - AND Random RT6600 ({}) - 200€", total_p2).expect("Error writing!");
                }
                if total_p3 > 0 {
                    writeln!(file, "Potato ({}) - 1€", total_p3).expect("Error writing!");
                }
                writeln!(file, "------------------------------").expect("Error writing!");
                writeln!(file, "Final price: {}", total).expect("Error writing!");
                writeln!(file, "------------------------------").expect("Error writing!");
                println!("Thank you for your purchase!");
                break;
            }
            _ => {
                println!("wrong input");
            }
        }
    }
}
