use std::{
    collections::HashMap,
    io::{self, Write},
};

use time::OffsetDateTime;
use uuid::Uuid;

fn main() {
    // Create/Inicialize Inventory
    let mut inventory = Inventory::new();

    let first_product = Product {
        id: Uuid::new_v4(),
        name: "Phone S".to_string(),
        brand: Brand::Samsung,
        price: 1000.5,
        stock: 10,
        updated_at: OffsetDateTime::now_local().unwrap(),
    };

    inventory.products.insert(first_product.id, first_product);

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("========== INVENTORY =========");
        println!("1. See all");
        println!("2. Search");
        println!("3. Create");
        println!("4. Update");
        println!("5. Delete");
        println!("6. Exit");

        /*
        print! macro doesn't automatically flush (clear) the output buffer,
        unlike println! which does include a newline and flushes automatically.
        */
        print!("--> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u8 = input.trim().parse().unwrap_or(0);
        // Error handler
        // let option: u8 = match option.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Invalid option. Please enter a valid number.");
        //         println!("Enter to continue...");
        //         io::stdin().read_line(&mut String::new()).unwrap();
        //         continue; // Next loop iteration
        //     }
        // };

        match input {
            1 => {
                print!("\x1B[2J\x1B[1;1H");
                inventory.see_all();
                println!("Enter to continue...");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            2 => {
                print!("\x1B[2J\x1B[1;1H");
                print!("Search by name: ");
                io::stdout().flush().unwrap();
                let mut query: String = String::new();
                io::stdin().read_line(&mut query).unwrap();

                inventory.search_by_name(query.trim());

                println!("Enter to continue...");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            3 => {
                let (name, brand, price, stock) = handler_input_data();
                inventory.create(&name, brand, price, stock);
            }
            4 => {
                let id = handler_input_id();

                let (name, brand, price, stock) = handler_input_data();
                let new_data = ProductDto {
                    name,
                    brand,
                    price,
                    stock,
                };
                inventory.update(id, new_data);
            }
            5 => {
                let id = handler_input_id();
                inventory.delete(id);
            }
            6 => break,
            _ => println!("Invalid input."),
        }
    }
}

// ENUM, STRUCTS AND IMPL
#[derive(Debug)]
enum Brand {
    Apple,
    Google,
    Samsung,
}

#[derive(Debug)]
struct Product {
    id: Uuid,
    name: String,
    brand: Brand,
    price: f32,
    stock: u16,
    updated_at: OffsetDateTime,
}

struct ProductDto {
    name: String,
    brand: Brand,
    price: f32,
    stock: u16,
}

struct Inventory {
    products: HashMap<Uuid, Product>,
}

// self: Would take complete ownership of the instance (consuming it)
// &self: Immutable reference to the instance
// &mut self: Mutable reference to the instance

impl Inventory {
    /// Create and return an instance of Inventory with empty products hashmap
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    // &self (immutable reference)
    // This method only needs to read data from Inventory without modifying it
    // Does not allow modifying the Inventory structure or its contents
    // Allows other methods or parts of the code to continue reading the data simultaneously
    fn see_all(&self) {
        //println!("Products: {:?}", self.products);

        if self.products.is_empty() {
            return println!("No products");
        }

        let mut counter: u16 = 0;
        for product in self.products.values() {
            let uuid_str = product.id.to_string();
            counter += 1;

            println!(
                "{}. ID: {}. Name: {}, brand: {:?}, price {}, stock: {}, updated at: {}",
                counter,
                &uuid_str[0..8],
                product.name,
                product.brand,
                product.price,
                product.stock,
                product.updated_at
            )
        }
    }

    fn search_by_name(&self, query: &str) {
        if self.products.is_empty() {
            return println!("No products");
        }

        for item in self.products.values() {
            if item.name.to_lowercase().contains(&query.to_lowercase()) {
                return println!(
                    "ID: {}. Name: {} - Brand: {:?}.",
                    item.id, item.name, item.brand
                );
            }
        }

        println!("'{}' not found.", query);
    }

    /// &str is only a temporary reference
    /// to avoid copying a String unnecessarily.
    /// It remains a reference until to_string() is called.
    /// Brand, price, and stock move directly.
    fn create(&mut self, name: &str, brand: Brand, price: f32, stock: u16) {
        let new = Product {
            id: Uuid::new_v4(),
            name: name.to_string(),
            brand,
            price,
            stock,
            updated_at: OffsetDateTime::now_local().unwrap(),
        };

        self.products.insert(new.id, new);
    }

    // &mut self (mutable reference)
    // Indicates it needs to modify the data
    // Allows making modifications to the structure and its contents
    // Ensures no other part of the code is reading or modifying the Inventory
    // while this method is executing
    fn update(&mut self, id: Uuid, new_data: ProductDto) {
        if let Some(product) = self.products.get_mut(&id) {
            //if let Some(new_price) = price {
            //    product.price = new_price;
            //}

            product.name = new_data.name;
            product.brand = new_data.brand;
            product.price = new_data.price;
            product.stock = new_data.stock;
            product.updated_at = OffsetDateTime::now_local().unwrap();
        }
    }

    fn delete(&mut self, id: Uuid) {
        //self.products.remove(&id);

        if self.products.remove(&id).is_some() {
            println!("Product with ID {} deleted.", id);
        } else {
            println!("Product not found.");
        }
    }
}

// HELPERS

// read_line(&mut input)
// The read_line() method needs to modify the String's content (input),
// but doesn't want to take ownership of the variable.
// The mutable reference (&mut)
// allows a function to modify the value it points to without taking ownership.
//
// If read_line() took input directly (without a reference),
// the function would take ownership of the String and the code couldn't use that variable afterward.
//
// If it took an immutable reference (&input), it couldn't modify the String's content.
//
// In other words:
// - mut input, allows YOU to modify the variable in your code
// - &mut input, allows A FUNCTION to modify the content without taking ownership
//
// This is part of Rust's ownership and borrowing system,
// which guarantees memory safety without needing a garbage collector.

fn handler_input_data() -> (String, Brand, f32, u16) {
    print!("\x1B[2J\x1B[1;1H");
    println!("New product");

    // Name
    let mut name: String = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read NAME");
    let name: String = name.trim().to_string();

    // Brand
    let brand = loop {
        let mut brand_input = String::new();
        print!("Brand (Apple, Samsung or Google): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut brand_input)
            .expect("Failed to read BRAND");
        let brand_input = brand_input.trim().to_lowercase();

        match brand_input.as_str() {
            "apple" => break Brand::Apple,
            "google" => break Brand::Google,
            "samsung" => break Brand::Samsung,
            _ => println!("Invalid brand! Please enter Apple, Google or Samsung."),
        }
    };

    // Price
    let price = loop {
        let mut price_input = String::new();
        print!("Price: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut price_input)
            .expect("Failed to read PRICE");

        match price_input.trim().parse::<f32>() {
            Ok(p) if p > 0.0 => break p,
            _ => println!("Invalid price! Enter a positive number."),
        }
    };

    // Stock
    let stock = loop {
        let mut stock_input = String::new();
        print!("Stock: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut stock_input)
            .expect("Failed to read STOCK");

        match stock_input.trim().parse::<u16>() {
            Ok(s) => break s,
            _ => println!("Invalid stock! Enter a positive whole number."),
        }
    };

    (name, brand, price, stock)
}

fn handler_input_id() -> Uuid {
    loop {
        let mut id_input = String::new();
        print!("Product ID: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut id_input).unwrap();

        match id_input.trim().parse::<Uuid>() {
            Ok(id) => break id,
            Err(_) => println!("Invalid ID! Please enter a valid UUID."),
        }
    }
}
