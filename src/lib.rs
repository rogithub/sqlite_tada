use rusqlite::{Connection, Result, Error};

pub fn create_table(conn: Connection) -> Result<usize, Error> {
    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users ('Alice', 42);
        INSERT INTO users ('Bob', 69);
    ";
    conn.execute(&query, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table_test() {
        let conn = Connection::open_in_memory().unwrap();
        let result = create_table(conn);
        assert_eq!(result, Ok(0));
    }
}
