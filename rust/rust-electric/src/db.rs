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
    pub name: String,
    pub age: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
}

fn get_conn() -> PgConnection {
    PgConnection::establish("postgres://postgres:password@localhost:5432/demo").unwrap()
}

pub fn init_db() {
    create_table();
    fill_values();
}

fn create_table() {
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS person (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            age INT
        )",
    )
    .execute(&mut get_conn())
    .expect("Error initiating tables");
}

fn fill_values() {
    let num_of_records: i64 = person::dsl::person
        .count()
        .get_result(&mut get_conn())
        .unwrap();

    if num_of_records == 0 {
        add_person(NewPerson {
            name: "John".to_string(),
            age: None,
        });
    }
}

pub fn add_person(new_person: NewPerson) {
    diesel::insert_into(person::table)
        .values(&new_person)
        .execute(&mut get_conn())
        .expect("Error adding person");
}

pub fn delete_person(id: i32) {
    use person::dsl;
    diesel::delete(dsl::person.filter(dsl::id.eq(id)))
        .execute(&mut get_conn())
        .expect("Error deleting person");
}
