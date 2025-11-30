// SQL Integration Tests for GUL
// Tests SQL query execution, type mapping, and database operations

#[cfg(test)]
mod sql_integration_tests {
    use gul::interop::sql::SqlBridge;

    #[test]
    fn test_sql_bridge_creation() {
        let bridge = SqlBridge::new();
        assert!(bridge.is_ok(), "SqlBridge should be created successfully");
    }

    #[test]
    fn test_basic_select_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "SELECT * FROM users WHERE age > 18";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "Basic SELECT query should be valid");
    }

    #[test]
    fn test_insert_query() {
        let bridge = SqlBridge::new().unwrap();
        let query =
            "INSERT INTO users (name, email, age) VALUES ('Alice', 'alice@example.com', 25)";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "INSERT query should be valid");
    }

    #[test]
    fn test_update_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "UPDATE users SET age = 26 WHERE name = 'Alice'";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "UPDATE query should be valid");
    }

    #[test]
    fn test_delete_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "DELETE FROM users WHERE age < 18";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "DELETE query should be valid");
    }

    #[test]
    fn test_join_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "SELECT u.name, o.total FROM users u JOIN orders o ON u.id = o.user_id";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "JOIN query should be valid");
    }

    #[test]
    fn test_aggregate_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "SELECT COUNT(*), AVG(age), MAX(age), MIN(age) FROM users GROUP BY country";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "Aggregate query should be valid");
    }

    #[test]
    fn test_subquery() {
        let bridge = SqlBridge::new().unwrap();
        let query =
            "SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 100)";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "Subquery should be valid");
    }

    #[test]
    fn test_invalid_syntax() {
        let bridge = SqlBridge::new().unwrap();
        let query = "SELCT * FRM users"; // Intentional typos

        let result = bridge.validate_query(query);
        assert!(result.is_err(), "Invalid SQL syntax should be rejected");
    }

    #[test]
    fn test_type_mapping_integer() {
        let bridge = SqlBridge::new().unwrap();
        let sql_type = "INTEGER";

        let gul_type = bridge.map_result_type(sql_type);
        assert_eq!(gul_type, "int", "INTEGER should map to int");
    }

    #[test]
    fn test_type_mapping_varchar() {
        let bridge = SqlBridge::new().unwrap();
        let sql_type = "VARCHAR";

        let gul_type = bridge.map_result_type(sql_type);
        assert_eq!(gul_type, "str", "VARCHAR should map to str");
    }

    #[test]
    fn test_type_mapping_boolean() {
        let bridge = SqlBridge::new().unwrap();
        let sql_type = "BOOLEAN";

        let gul_type = bridge.map_result_type(sql_type);
        assert_eq!(gul_type, "bool", "BOOLEAN should map to bool");
    }

    #[test]
    fn test_type_mapping_float() {
        let bridge = SqlBridge::new().unwrap();
        let sql_type = "REAL";

        let gul_type = bridge.map_result_type(sql_type);
        assert_eq!(gul_type, "float", "REAL should map to float");
    }

    #[test]
    fn test_parameterized_query() {
        let bridge = SqlBridge::new().unwrap();
        let query = "SELECT * FROM users WHERE name = ? AND age > ?";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "Parameterized query should be valid");
    }

    #[test]
    fn test_create_table() {
        let bridge = SqlBridge::new().unwrap();
        let query = "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(100), age INTEGER)";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "CREATE TABLE should be valid");
    }

    #[test]
    fn test_drop_table() {
        let bridge = SqlBridge::new().unwrap();
        let query = "DROP TABLE IF EXISTS users";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "DROP TABLE should be valid");
    }

    #[test]
    fn test_transaction_begin() {
        let bridge = SqlBridge::new().unwrap();
        let query = "BEGIN TRANSACTION";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "BEGIN TRANSACTION should be valid");
    }

    #[test]
    fn test_transaction_commit() {
        let bridge = SqlBridge::new().unwrap();
        let query = "COMMIT";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "COMMIT should be valid");
    }

    #[test]
    fn test_transaction_rollback() {
        let bridge = SqlBridge::new().unwrap();
        let query = "ROLLBACK";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "ROLLBACK should be valid");
    }

    #[test]
    fn test_index_creation() {
        let bridge = SqlBridge::new().unwrap();
        let query = "CREATE INDEX idx_users_email ON users(email)";

        let result = bridge.validate_query(query);
        assert!(result.is_ok(), "CREATE INDEX should be valid");
    }
}
