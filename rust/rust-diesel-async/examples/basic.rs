use std::error::Error;

use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use rust_diesel_async::{
    create_pool,
    gen::schema::employees::{dsl as employee_col, dsl::employees},
};

#[derive(Queryable, Debug, Selectable)]
#[diesel(table_name = rust_diesel_async::gen::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Employee {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = create_pool("postgres://postgres:password@localhost:5432/demo")?;
    let mut connection = pool.get().await?;

    insert_into(employees)
        .values((employee_col::name.eq("Jack"), employee_col::age.eq(21)))
        .execute(&mut connection)
        .await?;

    let employees_older_than_10: Vec<Employee> = employees
        .select(Employee::as_select())
        .filter(employee_col::age.gt(10))
        .load(&mut connection)
        .await?;

    employees_older_than_10.iter().for_each(|x| {
        println!(
            "{1}({0}) is older than 10, actual age: {2:?}",
            x.id, x.name, x.age
        );
    });
    Ok(())
}
