use std::{future::{Ready, ready}};

use actix_web::{Error, HttpResponse, Responder};
use serde::Serialize;


/**
 * contains person data and is serializable using serde
 */
#[derive(Serialize)]
pub struct Person<'this> {
    name: &'this str,
    phrase: &'this str,
    age: i8
}

/**
 * convert struct Person into HttpResponse by implementing Responder
 */
impl <'this> Responder for Person<'this> {
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
impl <'this> Person<'this> {
    pub fn new(name: &'this str, phrase: &'this str, age: i8) -> Self {
        Person {name, phrase, age}
    }
}