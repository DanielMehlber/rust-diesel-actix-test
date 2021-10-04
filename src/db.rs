use diesel::{Connection, ConnectionError, mysql};

pub fn establish_connection<'method>(host: &'method str, port: u16, database: &'method str,user: &'method str, password: &'method str) -> Result<mysql::MysqlConnection, ConnectionError> {
    mysql::MysqlConnection::establish(format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database).as_str())
}