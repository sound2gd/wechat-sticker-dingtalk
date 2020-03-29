use actix_web::{web, Error};
use futures::StreamExt;

pub async fn extract_body(mut body: web::Payload) -> Result<String, Error> {
  let mut bytes = web::BytesMut::new();
  while let Some(item) = body.next().await {
    bytes.extend_from_slice(&item?);
  }

  match String::from_utf8(bytes.to_vec()) {
    Ok(r) => Ok(r),
    Err(e) => Err(Error::from(e.utf8_error())),
  }
}
