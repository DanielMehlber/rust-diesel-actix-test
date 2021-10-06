// this is necessary
#[macro_use] extern crate diesel;

use actix_web::{App, HttpResponse, HttpServer, get, web};
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl, r2d2::{ConnectionManager, Pool}};
use person::Person;

// imports important traits (they must be in scope in order to work)
use self::diesel::prelude::*;

mod person;
mod db;
mod schema;

/**
 * Actix-Web Application data passed into API functions via web::Data<MyAppData> parameter
 */
struct MyAppData {
    // connection pool instead of raw-connection
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

/**
 * just test api
 */
#[get("/index")]
async fn index() -> &'static str {
    "Es funktioniert"
}

/**
 * create some person data and send it in JSON format
 */
#[get("/person")]
async fn get_person() -> person::Person {
    person::Person::new(String::from("Daniel Mehlber"), String::from("This is a test program"), 19)
}

/**
 * search for user in database
 */
#[get("/search/{name}")]
async fn search_person(web::Path(search): web::Path<String>, data: web::Data<MyAppData>) -> HttpResponse {
    //                 ^^^^^^^^^^^^^^^^^ path params         ^^^^ application data set in setup
    let connection = data.pool.get().unwrap();

    // *::dsl contains helper types and bare functions not contained in prelude
    use schema::person::dsl::{name, person};

    let people = person.filter(name.like(format!("%{}%", search))).load::<Person>(&connection);

    match people {
        Ok(list) => match list.get(0) {
            Some(pers) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(pers).unwrap()),
            None => HttpResponse::NoContent().body("no person found")
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

/*
 * Run server and mount API
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // connection to database established
    let pool = match db::establish_connection("localhost", 3306, "Home", "read-user", "password123") {
        Ok(conn) => conn,
        Err(err) => panic!("{}", err.to_string())
    };


    HttpServer::new(move || { 
        App::new()  .service(index)
                    .service(get_person)
                    .service(search_person)
                    .data(MyAppData {pool: pool.clone()}) // set application data (used in API functions)
    }).bind("localhost:8080")?.run().await
}