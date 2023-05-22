#![allow(unused_doc_comments)]

use crate::schema::dishes;
use crate::schema::meals;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

/// Dish struct to represent a dish in the database
#[derive(Queryable, Serialize, Deserialize)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,
}

/// Struct to represent a new dish to be inserted into the database
#[derive(Insertable, Deserialize)]
#[diesel(table_name = dishes)]
pub struct NewDish {
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,

}

/// Struct that represents a dish reqested by the user, either for creation or deletion
///
/// The serving size is optional, and defaults to 100g
#[derive(Deserialize)]
pub struct ReqDish {
    pub name: String,
}



/// Meal struct to represent a meal in the database
#[derive(Queryable, Serialize, Deserialize)]
pub struct Meal {
    pub meal_id: i32,
    pub name: String,
    pub appetizer: Option<i32>,
    pub main: Option<i32>,
    pub dessert: Option<i32>,
    pub cal: Option<f64>,
    pub sodium: Option<f64>,
    pub sugar: Option<f64>,
}

/// Struct to represent a new meal to be inserted into the database
#[derive(Insertable, Deserialize)]
#[diesel(table_name = meals)]
pub struct NewMeal {
    pub name: String,
    pub appetizer: Option<i32>,
    pub main: Option<i32>,
    pub dessert: Option<i32>,
}
