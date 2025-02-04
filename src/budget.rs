use crate::models::{Income, Expense};


pub fn calculate_budget(expenses: &Vec<Expense>, income: &Vec<Income>) -> f64 {
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
