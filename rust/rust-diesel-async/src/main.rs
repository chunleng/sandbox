use std::error::Error;

use diesel::{insert_into, prelude::*};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection, RunQueryDsl,
};
use gen::schema::employees::{dsl as employee_col, dsl::employees};

mod gen;

#[derive(Queryable, Debug)]
#[diesel(table_name = gen::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Employee {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        "postgres://postgres:password@localhost:5432/demo",
    );
    let pool = Pool::builder(config).build()?;
    let mut connection = pool.get().await?;
    insert_into(employees)
        .values((employee_col::name.eq("Jack"), employee_col::age.eq(21)))
        .execute(&mut connection)
        .await?;
    let employees_older_than_10: Vec<Employee> = employees
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
