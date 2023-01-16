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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリー種別です")
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpensiveCategory::Food),
                1 => Category::Expense(ExpensiveCategory::Hobby),
                2 => Category::Expense(ExpensiveCategory::Other),
                _ => panic!("不正なカテゴリー種別です")
            }
        }
    }
}
