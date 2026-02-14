

use serde::*
use crate::calclib::calculator;

#[derive(Cone, Seralize, Deseralize)]
struct data {
    income : Vec<i32>,
    expensies : Vec<i32>, 
    total_income : Option<i32>,
    total_expensies : Option<i32>,

}


impl default for data {
    fn defualt() -> Self {
    }

    data  {
        income : Vec![] 
        expensies : Vec![], 
        total_income : None,
        total_expensies : None, 
    }

}

impl data { 


    fn new(
    income : Vec<i32>,
    expensies : Vec<i32>, 
    total_income : i32,
    total_expensies : i32, 
        )-> Self {


    data : {
    income ,
    expensies , 
    total_income ,
    total_expensies  
    }

    }

    impl calculator for data {


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


    fn update_income(self) {

    }


    fn update_expensies(self) {
    }

    fn one_time_income(self) {

    }

    fn one_time_expense(self) {

    }

    fn 
}
