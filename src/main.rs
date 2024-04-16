// src/main.rs

use rocket::*;
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, ConnectionTrait, Database, DbBackend, Statement};
use futures::executor::block_on;
use sea_orm_migration::prelude::*;
use migrator::Migrator;
use crate::setup::set_up_db;

mod setup;
mod migrator;


#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}
/*
#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;
    
    let bakery_names = Bakery::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();
       
        
    Json(bakery_names)
} 
*/
async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    let db = db as &DatabaseConnection;
    
    let schema_manager = SchemaManager::new(db); // To investigate the schema
    
    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);    
    
    Ok(())
}


#[launch] // The "main" function of the proram
async fn rocket() -> _ {
    let db = match set_up_db().await {
     Ok(db) => db,
     Err(err) => panic!("{}", err),
    }; 
    
    if let Err(err) = block_on(run(&db)) {
        panic!("{}", err);
    }

    rocket::build()
        .manage(db)
        .mount(
            "/", 
            routes![
                index, 
               //  bakeries
            ]
        )
}

