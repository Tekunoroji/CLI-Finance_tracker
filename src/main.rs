use std::io;
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::{Local};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Expense {
    name: String,
    amount: f64,
    category: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Income {
    name: String,
    amount: f64,
}

fn main() { 

    let mut income_all: Vec<Income> = Vec::new();
    let mut expense_all: Vec<Expense> = Vec::new();

    let date_today = Local::now().format("%A, %d %B %Y");

    //Read the Data which was saved in a previous session 
    let (expense_saved, income_saved) = read_transactions();

    income_all.extend(income_saved);
    expense_all.extend(expense_saved);

    let mut budget = calculate_budget(&expense_all, &income_all);
  
    loop {
        println!("==========================================");
        println!("||             CLI Finance              ||");
        println!("==========================================");
        println!("======- {} -========", date_today);
        println!("==========================================");
        println!("Choose an option by typing the number e.g. 1");
        println!("__________________________________________");
        println!("Budget : ${}", budget);
        println!("1. Add Expense");
        println!("2. Add Income");
        println!("3. Show Transactions");
        println!("4. Exit");

        let selection_input = readline();

        match selection_input.trim() {
            "1" => {
                let expense = create_expense();
                println!("Your Expense {} - ${} with category {} was added", expense.name, expense.amount, expense.category);
                expense_all.push(expense);
                budget = calculate_budget(&expense_all, &income_all);
               
            }
            "2" => {
                let income = create_income();
                println!("Added {} with ${} to your income",income.name, income.amount);
                income_all.push(income);
                budget = calculate_budget(&expense_all, &income_all);
            }
            "3" => {

                println!("==========================================");
                println!("||              Expenses                ||");
                println!("==========================================");

                for expense in &expense_all {
                    println!("{}, {} category: {}",expense.name, expense.amount, expense.category);
                }

                println!("==========================================");
                println!("||              Income                  ||");
                println!("==========================================");

                for income in &income_all {
                    println!("{}, {}", income.name, income.amount);
                }
            }
            "4" => {
                println!("Exiting Program...");
                save_exit(&expense_all, &income_all);
                break
            }
            _ => println!("Invalid Input, Please choose a number from the menu!"),
        }
    }
}

//Helper Function to get input form the console

fn readline() -> String {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Line");
    
    input
}

//Helper function to get Income

fn get_amount() -> f64 {
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

fn create_expense() -> Expense {
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

fn create_income() -> Income {
    println!("====Please input the name of your income=====");

    let income_name_input = readline().trim().to_string();

    let income_amount = get_amount();

    Income {
        name: income_name_input,
        amount: income_amount,
    }
}

//Function to save the Transactions as JSON Data, so the entrys are not lost when we close the program.

fn save_exit(expenses: &Vec<Expense>, income: &Vec<Income>) {

    let transactions = (expenses, income);

    let serialized_transactions = serde_json::to_string(&transactions).unwrap();

    fs::write("transactions.json", serialized_transactions).expect("Unable to write File");
}

fn read_transactions() -> (Vec<Expense>, Vec<Income>) {
    let transactions = fs::read_to_string("transactions.json").expect("Unable to read file");
    let deserialized_transactions: (Vec<Expense>, Vec<Income>) = serde_json::from_str(&transactions).unwrap_or_else(|_| (vec![], vec![]));
    deserialized_transactions
}

//Function to calculate and display the budget

fn calculate_budget(expenses: &Vec<Expense>, income: &Vec<Income>) -> f64 {
    let mut income_all_amount: Vec<f64> = Vec::new();
    let mut expense_all_amount: Vec<f64> = Vec::new();

    for position in income {
        income_all_amount.push(position.amount)
    };

    for position in expenses {
        expense_all_amount.push(position.amount)
    };

    income_all_amount.iter().sum::<f64>() - expense_all_amount.iter().sum::<f64>()
}
