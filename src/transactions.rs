use crate::models::{Income, Expense};
use std::io;
use std::fs;

pub struct Transactions {
    pub income_all: Vec<Income>,
    pub expense_all:  Vec<Expense>,
    
}

impl Transactions {
    pub fn new() -> Self {
        let (expense_saved, income_saved) = read_transactions();
        Self {
            income_all: income_saved,
            expense_all: expense_saved,
        }
    }

    pub fn save(&self) {
        save_exit(&self.expense_all, &self.income_all);
    }
}



//Helper function to get Income

pub fn get_amount() -> f64 {
    loop {
        println!("=====Please input the amount======");

        let amount_input = readline();

        match amount_input.trim().parse::<f64>() {
            Ok(num) => {
                if num > 0.0 {
                    return num;
                } else {
                    println!("Number must be bigger then 0");
                }
            }
            Err(_) => println!("Please input a valid number"),
        }
    }
}

//Helper Function to create a expense struc

pub fn create_expense() -> Expense {
    println!("=====Please input the name of your expense=====");
    let expense_input = readline().trim().to_string();

    let expense_amount_input = get_amount();

    println!("=====Please input the category of your expense=====");

    let expense_category_input = readline().trim().to_string();
    
    Expense {
        name: expense_input,
        amount: expense_amount_input,
        category: expense_category_input,
    }    
}

//Function to create my Income Struct

pub fn create_income() -> Income {
    println!("====Please input the name of your income=====");

    let income_name_input = readline().trim().to_string();

    let income_amount = get_amount();

    Income {
        name: income_name_input,
        amount: income_amount,
    }
}

//Function to save the Transactions as JSON Data, so the entrys are not lost when we close the program.

pub fn save_exit(expenses: &Vec<Expense>, income: &Vec<Income>) {

    let transactions = (expenses, income);

    let serialized_transactions = serde_json::to_string(&transactions).unwrap();

    fs::write("transactions.json", serialized_transactions).expect("Unable to write File");
}

//Function to read Transactions saved to the file transactions.json

pub fn read_transactions() -> (Vec<Expense>, Vec<Income>) {
    let transactions = fs::read_to_string("transactions.json").expect("Unable to read file");
    let deserialized_transactions: (Vec<Expense>, Vec<Income>) = serde_json::from_str(&transactions).unwrap_or_else(|_| (vec![], vec![]));
    deserialized_transactions
}

//Function to readline to easily reuse it

pub fn readline() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Line");

    input
}
