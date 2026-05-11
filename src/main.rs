// library for accepting user input in the terminal
use std::io::{self, Write};

// creating a structure to hold name and amount
#[derive(Debug)]
struct Bill{
    name: String,
    amount: f64,
}

// function to add bill 
fn add_bill(bills: &mut Vec<Bill>){

    println!("==Add Bill==");
    // print!() does not add new line in terminal
    print!("  Bill name : ");
    
    // user input the name 
    io::stdout().flush().expect("flush failed");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("read failed");
    let name = name.trim().to_string();
    
    // user input the amount as string and it is converted to integer through shadowing
    print!("  Amount    : Ksh ");
    io::stdout().flush().expect("flush failed");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("read failed");
    let amount: f64 = match amount.trim().parse() {
        Ok(n) => n,
        Err(_) => { println!("  Invalid amount."); return; }
    };
    // update the vector using .push() to add the elements from user input
    bills.push(Bill { name, amount });
    println!("  Bill added!");
}
    
// function to view bills
fn view_bill(bills: &Vec<Bill>){
    // to check if there is any bills if so to dispay them and if not to print the string no bills yet 
    println!("== View Bills ==");
    if bills.is_empty() {
        println!("  No bills yet.");
        return;
    }
    for bill in bills {
        println!("  {} — Ksh{}", bill.name, bill.amount);
    }
}

// function to remove bills
fn remove_bill(bills: &mut Vec<Bill>){
    println!("==Remove Bill==");
    
    // to check if bills is empty using .is_empty()
    if bills.is_empty() {
    println!("  No bills to remove.");
    return;
    }
    
    // Show existing bills with index numbers
    for (i, bill) in bills.iter().enumerate() {
    println!("  {}. {} — Ksh{}", i + 1, bill.name, bill.amount);
    }
    
    // remove the bill using the index number as the user input
    print!("  Enter bill number to remove: ");
    io::stdout().flush().expect("flush failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read failed");

    let index: usize = match input.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= bills.len() => n - 1,  // convert 1-based to 0-based
        _ => {
            println!("  Invalid number.");
            return;
        }
    };

    // .remove() removes the bill
    let removed = bills.remove(index);
    println!("  Removed: {}", removed.name);
    }


    // function to udate a 
fn update_bill(bills: &mut Vec<Bill>){
    println!("==Update Bill==");

    // if no bills return no bills to update
    if bills.is_empty() {
    println!("  No bills to update.");
    return;
    }

    // Show existing bills
    for (i, bill) in bills.iter().enumerate() {
        println!("  {}. {} — Ksh{}", i + 1, bill.name, bill.amount);
    }

    // input the bill number you want to update
    print!("  Enter bill number to update: ");
    io::stdout().flush().expect("flush failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read failed");

    let index: usize = match input.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= bills.len() => n - 1, 
        _ => {
            println!("  Invalid number.");
            return;
        }
    };

    // brings the index, name and amount of the bill selected
    println!("  Current: {} — Ksh{}", bills[index].name, bills[index].amount);

    // update the amount 
    print!("  New amount (Ksh): ");
    io::stdout().flush().expect("flush failed");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("read failed");

    // the string input is converted to float and updated the bill
    match amount.trim().parse::<f64>() {
        Ok(n) => {
            bills[index].amount = n;
            println!("  Updated: {} — Ksh{}", bills[index].name, bills[index].amount);
        }
        Err(_) => println!("  Invalid amount, no changes made."),
    }
}

// calculates the total amount of bills owed
fn bill_total(bills: &Vec<Bill>) {
        println!("==Total Bill==");
        let total: f64 = bills.iter().map(|b| b.amount).sum();
        println!("Total: ksh{}", total);
}
    

// the menu displayed in the terminal
fn menu_items(){
    // creating a new vector for storing values
    let mut bills: Vec<Bill> = Vec::new(); 

    // use loop to iterate the program and break to exit the loop using ctrl+c
    loop{
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total");
        
        // user input between 1 and 5 
        let mut choice = String::new();
        
        println!("Enter selection: {}",choice);
        io::stdin()
            .read_line(&mut choice)
            .expect("Choice out of scope");

        // convert the string input to integer
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // the number selected will coincide with the function it will do
        match choice{
            1 => add_bill(&mut bills),
            2 => view_bill(&bills),
            3 => remove_bill(&mut bills),
            4 => update_bill(&mut bills),
            5 => bill_total(&bills),
            _ => break,
        }
    }
}

// to run the code
fn main() {
    menu_items();
}