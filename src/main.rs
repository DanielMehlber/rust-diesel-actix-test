// this is necessary
#[macro_use] extern crate diesel;

use actix_files::{Files};
use actix_web::{App, HttpResponse, HttpServer, get, web};
use diesel::{MysqlConnection, r2d2::{ConnectionManager, Pool}};

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

    let people = db::search_users_in_database(search, &connection);

    match people {
        Ok(res) => match serde_json::to_string(&res) {
            Ok(list) => HttpResponse::Ok().content_type("application/json").body(list),
            Err(_) => HttpResponse::InternalServerError().body("")
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
        // serve API on /api/*
        let api_scope = web::scope("/api").service(index)
                                                .service(get_person)
                                                .service(search_person);  

        /*
         * VERY IMPORTANT HERE:
         * This application serves static files ('./') and the api ('./api')
         * 
         * First mount the api and THEN the static files because otherwise the paths will conflict.
         * 
         * See implementation notes:
         * "If the mount path is set as the root path /, services registered after this one will be inaccessible. Register more specific handlers and services first"
         *  
         */
        App::new()  .service(api_scope) // serve api
                    .service(Files::new("/", "./static").show_files_listing()) // serve static files of directory /static
                    .data(MyAppData {pool: pool.clone()}) // set application data (used in API functions)
    }).bind("localhost:8080")?.run().await
}