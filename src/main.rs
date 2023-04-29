// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::{io, collections::HashMap};
enum MainMenuItem {
    Addbill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: i32,
}

#[derive(Debug, Clone)]
pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    fn add_bill(&mut self, bill: Bill) {
       self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_bills(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove_bill(&mut self, name: &str) -> bool{
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: i32) -> bool {
        match self.inner.get_mut(name) {
            Some(mut bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

impl MainMenuItem {
    fn from_str(input: &str) -> Option<MainMenuItem> {
        match input {
            "1" => Some(MainMenuItem::Addbill),
            "2" => Some(MainMenuItem::ViewBill),
            "3" => Some(MainMenuItem::RemoveBill),
            "4" => Some(MainMenuItem::UpdateBill),
            _ => None,
        }    
    }

    fn show() {
        println!("");
        println!(" === Billing Information === ");
        println!("1. Add Billing Information");
        println!("2. View Billing Information");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter your choice:");
    }
}

fn get_user_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter a string.");
    }
    let input = buffer.trim().to_owned();
    println!("Input: {:?}", input);
    if input == ""{
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<i32> {
    println!("Enter bill amount");
    loop {
        let bill_amount = match get_user_input() {
            Some(input) => input,
            None => return None,
        };
        if &bill_amount == "" {
            return None;
        }
        let parsed_amount : Result<i32, _> = bill_amount.parse();
        match parsed_amount {
            Ok(amount ) => return Some(amount),
            Err(_) => println!("Please enter a number."),
        }
    }
}

mod menu {
    use crate::{Bills, get_user_input, Bill, get_bill_amount};

    pub fn add_bill(bills : &mut Bills) {
        println!("Enter bill name");
        let name = match get_user_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        let bill = Bill {
            name: name.to_owned(),
            amount,
        };
        bills.add_bill(bill);
        println!("Added bill");
    }

    pub fn view_bill(bills : &Bills) {
        for bill in bills.get_bills()  {
            println!("Viewing bill: {:?}", bill);
        }
    }

    pub fn remove_bill(bills : &mut Bills) {
        for bill in bills.get_bills()  {
            println!("Removing bill: {:?}", bill);
        }
        println!("Enter bill name to remove");
        let name = match get_user_input() {
            Some(input) => input,
            None => return,
        };
        if bills.remove_bill(&name) {
            println!("Bill removed successfully: {:?}", name);
        } else {
            println!("Removed bill");
        }
    }

    pub fn update_bill(bills : &mut Bills) {
        for bill in bills.get_bills()  {
            println!("Updating bill: {:?}", bill);
        }
        println!("Enter bill name to update");
        let name = match get_user_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Updated bill");
        } else {
            println!("Failed to update bill");
        }
    }

}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenuItem::show();
        let user_input = get_user_input()?;
        match MainMenuItem::from_str(&user_input) {
            Some(MainMenuItem::Addbill) => menu::add_bill(&mut bills),
            Some(MainMenuItem::ViewBill) => menu::view_bill(&mut bills),
            Some(MainMenuItem::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenuItem::UpdateBill) => menu::update_bill(&mut bills),
            //None => println!("No bill")
            None => break,
        }
    }
    None
}
fn main() {
    println!("Billing Application!");
    run_program();
}
