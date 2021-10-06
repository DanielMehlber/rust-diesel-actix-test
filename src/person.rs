use std::{future::{Ready, ready}};

use diesel::Queryable;

use actix_web::{Error, HttpResponse, Responder};
use serde::Serialize;


/**
 * contains person data and is serializable using serde
 */
#[derive(Serialize, Queryable)]
pub struct Person {
    pub name: String,
    pub phrase: String,
    pub age: i32
}

/**
 * convert struct Person into HttpResponse by implementing Responder
 */
impl Responder for Person {
    // error type that can be emitted
    type Error = Error;

    // result that will be emitted
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    // convert struct into result
    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))    
    }
}

/**
 * standard implementations for struct
 */
impl Person {
    pub fn new(name: String, phrase: String, age: i32) -> Self {
        Person {name, phrase, age}
    }
}