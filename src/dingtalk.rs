use actix_web::{client::Client, web, Error};
use futures::StreamExt;
use lazy_static::lazy_static;
use log::info;
use std::env::var;

lazy_static! {
  static ref DINGTALK_URL: String = format!(
    "https://oapi.dingtalk.com/robot/send?access_token={}",
    &var("DINGTALK_TOKEN").unwrap()
  );
}

// 发送钉钉请求
pub async fn send_msg(url: &str, client: &Client) -> Result<String, Error> {
  let bd = r#"{"msgtype": "image", "image": {"picURL": ""#.to_owned() + url + "\"}}";
  info!("use request-body, {}", bd);
  let mut res = client
    .post((*DINGTALK_URL).as_str())
    .header("Content-Type", "application/json")
    .send_body(&bd)
    .await
    .map_err(Error::from)?;

  let mut body = web::BytesMut::new();
  while let Some(chunk) = res.next().await {
    body.extend_from_slice(&chunk?);
  }

  match String::from_utf8(body.to_vec()) {
    Ok(r) => Ok(r),
    Err(e) => Err(Error::from(e.utf8_error())),
  }
}
