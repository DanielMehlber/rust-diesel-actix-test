use diesel::{Connection, ConnectionError, MysqlConnection, mysql, r2d2::{self, ConnectionManager, Pool}};

pub fn establish_connection<'method>(host: &'method str, port: u16, database: &'method str,user: &'method str, password: &'method str) -> Result<Pool<ConnectionManager<MysqlConnection>>, String> {
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database);
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(url);
    
    return match r2d2::Pool::builder()
        .build(manager) {
            Ok(pool) => Ok(pool),
            Err(err) => Err(format!("error while creating connection pool: {}", err.to_string()))
        }
}