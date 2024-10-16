use super::User;
use njord::condition::{Condition, Value};
use njord::keys::AutoIncrementPrimaryKey;
use njord::mysql;
use std::vec;

#[test]
fn delete_row() {
    insert_row();

    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let mut conn = mysql::open(url);

    let columns = vec!["username".to_string()];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("chasewillden2".to_string()),
    );

    match conn {
        Ok(ref mut c) => {
            let result = mysql::update(c, User::default())
                .set(columns)
                .where_clause(condition)
                .limit(4)
                .offset(0)
                .build();
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

/// Helper function to insert a row to be updated
fn insert_row() {
    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let mut conn = mysql::open(url);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden2".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mysql::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
