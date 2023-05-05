use rusqlite::{Connection, Result, Error, Params};

pub fn create_table(conn: &Connection, query: &str) -> Result<usize, Error> {    
    conn.execute(&query, ())
}

pub fn insert_record(conn: &Connection, query: &str) -> Result<usize, Error> {    
    conn.execute(&query, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_table_test() {
        let q_create = "CREATE TABLE users (name TEXT, age INTEGER);";
        //let q_insert = "INSERT INTO users ('Alice', 42);";
        let conn = Connection::open_in_memory().unwrap();
        let result = create_table(&conn, &q_create);
        assert_eq!(result, Ok(0));

        //let result1 = insert_record(&conn, &q_insert);
        //assert_eq!(result1, Ok(0));
    }
}
