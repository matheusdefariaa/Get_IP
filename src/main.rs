use actix_web::{get,web,App,HttpServer,HttpRequest,HttpResponse,Responder};
use actix_files::{self,NamedFile,Files};
use actix_web::dev::ConnectionInfo;
use tera::{Context, Tera};

fn configure_tera() -> Tera {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {eprintln!("Error {}",e);
    std::process::exit(1)}};
    tera
}

async fn index(tera: web::Data<Tera>, req: HttpRequest) -> impl Responder {
    let getip = get_ip(req).await;
    let mut context = Context::new();
    context.insert("IP",&getip);

    let rendered = tera.render("index.html",&context).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn get_ip(req: HttpRequest) -> String {
    let connection: &ConnectionInfo = &req.connection_info();
    let ip_real = connection.realip_remote_addr().unwrap_or("Unknown");
    ip_real.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(configure_tera()))
        .service(Files::new("/static","./static"))
        .service(Files::new("/assets","./assets"))
        .route("/",web::get().to(index))
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}