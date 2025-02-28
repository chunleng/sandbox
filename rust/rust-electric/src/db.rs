use diesel::prelude::*;
use serde::Deserialize;

table! {
    person (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        age -> Nullable<Int4>
    }
}

#[derive(Insertable)]
#[diesel(table_name = person)]
pub struct NewPerson {
    name: String,
    age: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
}

pub fn init_db() {
    let mut conn =
        PgConnection::establish("postgres://postgres:password@localhost:5432/demo").unwrap();

    create_table(&mut conn);
    fill_values(&mut conn);
}

fn create_table(conn: &mut PgConnection) {
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS person (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            age INT
        )",
    )
    .execute(conn)
    .expect("Error initiating tables");
}

fn fill_values(conn: &mut PgConnection) {
    let num_of_records: i64 = person::dsl::person.count().get_result(conn).unwrap();

    if num_of_records == 0 {
        let new_person = NewPerson {
            name: "John".to_string(),
            age: None,
        };

        diesel::insert_into(person::table)
            .values(&new_person)
            .execute(conn)
            .expect("Error filling initial values");
    }
}
