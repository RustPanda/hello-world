use actix_web::{
    get,
    post,
    delete,
    web,
    App,
    HttpServer,
    Responder,
    HttpResponse,
    Either,
    middleware,
};
use sled;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let my_data = web::Data::new(sled::open("my_db")?);

    println!("Server up!");


    HttpServer::new(move || App::new()
            .wrap(middleware::Logger::default())
            .app_data(my_data.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
