use actix_web::{client::Client, middleware, web, App, HttpServer, Responder};
use futures::future::FutureExt;
use log::info;
use serde_derive::Deserialize;
use serde_xml_rs::from_reader;

mod dingtalk;
mod util;
mod xml;

#[derive(Debug, Deserialize)]
struct WechatParam {
  signature: String,
  echostr: Option<String>,
  nonce: String,
}

async fn index(
  web::Query(inf): web::Query<WechatParam>,
  body: web::Payload,
  client: web::Data<Client>,
) -> impl Responder {
  if let Ok(bytes) = util::extract_body(body).await {
    info!("receive body: {:?}", bytes);

    match from_reader::<&[u8], xml::Xml>(bytes.as_bytes()) {
      Ok(mut xml) => {
        info!("xml: {:?}", xml);
        info!("current thread 1: {:?}", std::thread::current().id());

        actix_rt::spawn(async move {
          info!("current thread 2: {:?}", std::thread::current().id());
          match dingtalk::send_msg(xml.pic_url(), &client).await {
            Ok(r) => info!("result: {}", r),
            Err(e) => info!("error: {}", e),
          }
        });
      }
      Err(e) => info!("{}", e),
    }
  }

  match inf.echostr {
    Some(r) => r,
    None => "".to_owned(),
  }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();
  HttpServer::new(|| {
    App::new()
      .data(Client::default())
      // .data(ThreadPool::new().unwrap())
      .wrap(middleware::Logger::default())
      .service(
        web::resource("/wx")
          .route(web::get().to(index))
          .route(web::post().to(index)),
      )
  })
  .bind("127.0.0.1:3001")?
  .run()
  .await
}
