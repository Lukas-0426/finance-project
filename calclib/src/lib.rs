
use serde::{Serialize,Deserialize};
use Default;
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct FinanceData{
    income : Vec<i32>,
    expensies : Vec<i32>, 
    total_income : Option<i32>,
    total_expensies : Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Week {
    week : Vec<FinanceData>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Month {
    month : Vec<Week>
}

trait Calculator {

    fn calculate_balance() -> i32;

    fn update_income(income : i32) ;

    fn update_expensies(expense: i32); 

    fn one_time_income(income : i32);

    fn one_time_expense(expense : i32);
}
