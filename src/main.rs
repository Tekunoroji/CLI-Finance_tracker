mod budget;
mod models;
mod transactions;

use chrono::{Local};
use crate::budget::*;
use crate::transactions::*;



fn main() { 
    let date_today = Local::now().format("%A, %d %B %Y");
    let mut transactions = Transactions::new();

    let mut budget = calculate_budget(&transactions.expense_all, &transactions.income_all);
  
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
                transactions.expense_all.push(expense);
                budget = calculate_budget(&transactions.expense_all, &transactions.income_all);
               
            }
            "2" => {
                let income = create_income();
                println!("Added {} with ${} to your income",income.name, income.amount);
                transactions.income_all.push(income);
                budget = calculate_budget(&transactions.expense_all, &transactions.income_all);
            }
            "3" => {

                println!("==========================================");
                println!("||              Expenses                ||");
                println!("==========================================");

                for expense in &transactions.expense_all {
                    println!("{}, {} category: {}",expense.name, expense.amount, expense.category);
                }

                println!("==========================================");
                println!("||              Income                  ||");
                println!("==========================================");

                for income in &transactions.income_all {
                    println!("{}, {}", income.name, income.amount);
                }
            }
            "4" => {
                println!("Exiting Program...");
                transactions.save();
                break
            }
            _ => println!("Invalid Input, Please choose a number from the menu!"),
        }
    }
}

