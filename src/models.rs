use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Expense {
    pub name: String,
    pub amount: f64,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Income {
    pub name: String,
    pub amount: f64,
}
