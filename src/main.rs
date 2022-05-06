use std::process::exit;


use data::{init_db, new_db};
use data::todo::{TodoMac, TodoNew};

mod data;

#[tokio::main]
async fn main() {
    let db = match new_db().await {
        Ok(i) => i,
        Err(e) => {
            eprintln!("could not connect to database: {:?}", e);
            exit(1);
        }
    };


    init_db(&db).await.unwrap_or_else(|e| {
        eprintln!("could not initialize database: {:?}", e);
    });

    let new = TodoNew { title: "New Task".to_string(), description: "This is the description of the new task".to_string() } ;
    let tdm_execute = TodoMac::create(&db, new).await.unwrap_or_else(|e| {
        eprintln!("error executing create: {:?}", e);
        0
    });

    if tdm_execute == 0 {
        eprintln!("failed to create record");
    }


    let tdm = TodoMac::list(&db).await.unwrap_or_default();

    for t in tdm {
        println!("{:?}", t);
    }
}
