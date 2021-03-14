use actix_web::{
    get,
    // post,
    // delete,
    web,
    App,
    HttpServer,
    // Responder,
    // HttpResponse,
    // Either,
    middleware,
    Result
};
use sled;

#[get("/{key}")]
async fn get(data: web::Data<sled::Db>, web::Path((key,)): web::Path<(String,)>) -> Result<String> {
    match  data.get(key.as_bytes()).unwrap() {
        Some(value) => Ok(format!("Value: {}", String::from_utf8(value.to_vec()).unwrap())),
        None => Ok(format!("Not data"))
    }
}

#[get("/{key}/{value}")]
async fn push(data: web::Data<sled::Db>, web::Path((key, value)): web::Path<(String, String)>) -> Result<String> {
    match data.insert(key.as_bytes(), value.as_bytes()) {
        Ok(_) => Ok(String::from("Value write")),
        Err(e) => Ok(format!("Error: {}", e))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let my_data = web::Data::new(sled::open("my_db")?);

    println!("Server up!");

    HttpServer::new(move || App::new()
            .wrap(middleware::Logger::default())
            .app_data(my_data.clone())
            .service(get)
            .service(push)
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}