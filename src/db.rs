use diesel::{Connection, mysql};

pub fn establish_connection() -> mysql::MysqlConnection {
    let connection = mysql::MysqlConnection::establish("mysql://localhost:3306/Home?user=readuser").unwrap();

    connection
}