use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

// 収入の分類
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other
}

// 支出の分類
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpensiveCategory {
    Food,
    Hobby,
    Other
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpensiveCategory)
}

pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Item {name, category, price, date}
    }
}