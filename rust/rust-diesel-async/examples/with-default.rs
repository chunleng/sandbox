//! In this example, we want to take note how Insertable can use an extra Option to wrap into their
//! database type to set default or null (For nullable) or value

use std::error::Error;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rust_diesel_async::{
    create_pool,
    gen::schema::employees::{dsl as employee_col, dsl::employees},
};

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = rust_diesel_async::gen::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Employee {
    role: Option<String>,
    status: String,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = rust_diesel_async::gen::schema::employees)]
struct NewEmployee {
    name: String,
    age: Option<i32>,
    role: Option<Option<String>>, // Nullable
    status: Option<String>,       // Non-nullable
}

#[derive(diesel::Insertable)]
#[diesel(table_name = rust_diesel_async::gen::schema::employees)]
struct AnotherEmployee {
    name: String,
    age: Option<i32>,
    role: Option<String>, // Nullable
    status: String,       // Non-nullable
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = create_pool("postgres://postgres:password@localhost:5432/demo")?;
    let mut connection = pool.get().await?;

    // Clean slate
    diesel::delete(employees).execute(&mut connection).await?;

    diesel::insert_into(employees)
        .values(NewEmployee {
            name: "Alice".into(),
            age: Some(30),
            role: Some(None), // Set none
            status: None,     // Set default
        })
        .execute(&mut connection)
        .await?;

    let alice: Employee = employees
        .filter(employee_col::name.eq("Alice"))
        .select(Employee::as_select())
        .first(&mut connection)
        .await?;
    dbg!(&alice);
    assert_eq!(alice.role, None);
    assert_eq!(alice.status, "active");

    diesel::insert_into(employees)
        .values(NewEmployee {
            name: "Bob".into(),
            age: Some(31),
            role: None,   // set default
            status: None, // set default
        })
        .execute(&mut connection)
        .await?;

    let bob: Employee = employees
        .filter(employee_col::name.eq("Bob"))
        .select(Employee::as_select())
        .first(&mut connection)
        .await?;
    dbg!(&bob);
    assert_eq!(bob.role, Some("staff".into()));
    assert_eq!(bob.status, "active");

    diesel::insert_into(employees)
        .values(AnotherEmployee {
            name: "Charlie".into(),
            age: Some(31),
            role: None,               // set default (Never able to set null)
            status: "unknown".into(), // Never able to set default
        })
        .execute(&mut connection)
        .await?;

    let charlie: Employee = employees
        .filter(employee_col::name.eq("Charlie"))
        .select(Employee::as_select())
        .first(&mut connection)
        .await?;
    dbg!(&charlie);
    assert_eq!(charlie.role, Some("staff".into()));
    assert_eq!(charlie.status, "unknown");

    Ok(())
}
