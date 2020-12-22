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
            .app_data(my_data.clone())
            .service(get)
            .service(set)
            .service(delete)
            .service(delete_tree))

        .bind("127.0.0.1:8080")?
        .run()
        .await
}
#[get("/{table}/{id}")]
async fn get(web::Path((key_tree, id)): web::Path<(String, String)>,
               db: web::Data<sled::Db>) -> impl Responder {

    if let Ok(tree) = db.open_tree(key_tree.as_bytes()) {
        if let Some(value) = tree.get(id.as_bytes()).unwrap() {
            return Either::A(
                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(String::from_utf8( value.to_vec()).unwrap())
            )
        }
    }

    Either::B(
        HttpResponse::NotFound()
    )


}

#[post("/{key_tree}/{id}/{value}")]
async fn set(web::Path((key_tree, id, value)): web::Path<(String, String, String)>,
               db: web::Data<sled::Db>) -> impl Responder {
    if let Ok(tree) = db.open_tree(key_tree.as_bytes()) {
        if let Ok(_) = tree.insert(id.as_bytes(), value.as_bytes()) {
            return Either::A(HttpResponse::Created().finish())
        }
    }

    Either::B(HttpResponse::NotFound())
}

#[delete("/{key_tree}/{id}")]
async fn delete(web::Path((key_tree, id)): web::Path<(String, String)>,
             db: web::Data<sled::Db>) -> impl Responder {
    if let Ok(tree) = db.open_tree(key_tree.as_bytes()) {
        if let Some(_) = tree.remove(id.as_bytes()).unwrap() {
            return Either::A(HttpResponse::Ok())
        }
    }

    Either::B(HttpResponse::NotFound())
}

#[delete("/{key_tree}")]
async fn delete_tree(web::Path(key_tree): web::Path<String>,
                db: web::Data<sled::Db>) -> impl Responder {
    if let Ok(true) = db.drop_tree(key_tree.as_bytes()) {
        return Either::A(HttpResponse::Ok())
    }

    Either::B(HttpResponse::NotFound())
}
