use actix_web::{App, HttpServer, get};

mod person;
mod db;

/**
 * just test api
 */
#[get("/index")]
async fn index() -> &'static str {
    "Es funktioniert"
}

/**
 * retreive some person data (automatically serialized into JSON)
 */
#[get("/person")]
async fn get_person<'method>() -> person::Person<'method> {
    person::Person::new("Daniel Mehlber", "This is a test program", 19)
}

/*
 * Run server and mount API
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let connection = db::establish_connection();

    HttpServer::new(|| { 
        App::new().service(index).service(get_person)
    }).bind("localhost:8080")?.run().await
}