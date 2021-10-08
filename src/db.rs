use diesel::{MysqlConnection, r2d2::{self, ConnectionManager, Pool}};

use crate::schema;
use diesel::prelude::*;
use crate::person::Person;

pub fn establish_connection<'method>(host: &'method str, port: u16, database: &'method str,user: &'method str, password: &'method str) -> Result<Pool<ConnectionManager<MysqlConnection>>, String> {
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(url);
    
    return match r2d2::Pool::builder()
        .build(manager) {
            Ok(pool) => Ok(pool),
            Err(err) => Err(format!("error while creating connection pool: {}", err.to_string()))
        }
}

/**
 * search users by name and return a vector of matches
 */
pub fn search_users_in_database(keyword: String, connection: &MysqlConnection) -> Result<Vec<Person>, diesel::result::Error> {
    // *::dsl contains helper types and bare functions not contained in prelude
    use schema::person::dsl::*;

    person.filter(name.like(format!("%{}%", keyword))).load::<Person>(connection)
}