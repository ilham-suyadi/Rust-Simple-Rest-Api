use std::sync::Mutex;
use actix_web::web;
use crate::service::{AppState, Blog};

pub fn init_state()-> web::Data<AppState>{
    web::Data::new(AppState {
        blogs: Mutex::new(vec![
            Blog {
                id: 1,
                title: String::from("test"),
                content: String::from("lorem")
            },
            Blog {
                id: 2,
                title: String::from("rust"),
                content: String::from("dolar"),
            },
        ]),
 })
}