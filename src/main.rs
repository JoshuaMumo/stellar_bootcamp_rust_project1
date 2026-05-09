use std::io;

fn main() {
    println!("== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total");

    let mut choice = String::new();
    
    println!("Enter selection: {}",choice);
    io::stdin()
        .read_line(&mut choice)
        .expect("Choice out of scope");

}