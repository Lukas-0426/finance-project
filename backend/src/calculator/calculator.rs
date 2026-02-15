

use serde::*;
use crate::calclib::{Calculator,FinanceData, Week, Month};

impl FinanceData{ 

    fn new(
    income : Vec<i32>,
    expensies : Vec<i32>, 
    total_income : i32,
    total_expensies : i32, 
        )-> Self {


    FinanceData{
    income ,
    expensies , 
    total_income ,
    total_expensies  
    }

    }
}

    impl calculator for FinanceData{


    fn calculate_balance(&self) -> i32 {
       
        let calculated_income = 0;
        for x in self.income {
            calculated_income = calculated_income + x; 
        }

        let calculated_expense = 0;
        for x in self.expense {
            calculated_expense = calculated_expense + x; 
        }

        if(self.total_income != calcualted_income) {
            self.total_income = calculated_income; 
        }


        if(self.total_expense != calcualted_expense) {
            self.total_expensies = calculated_expense; 
        }

        self.total_income - self.total_expensies

       
    }


    fn update_income(&mut self, field_to_update : i32, income: i32) {

            for x in self.income {
                if field_to_update == x {
                    self.income[x] = income; 
                    return;
                }
            }
                
            self.income = income; 
    }


    fn update_expensies(self, expense : i32) {

            for x in self.expenseies {
                if field_to_update == x {
                    self.expenseies[x] = expense; 
                    return;
                }
            }
                
            self.expenseies = expense; 

    }

    fn one_time_income(self, income : i32) {
        
        self.income.push(income);
    }

    fn one_time_expense(self, expense : i32) {

        self.expensies.push(expense) 

    }

}
