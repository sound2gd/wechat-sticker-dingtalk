use serde_derive::Deserialize;

// 消息id，64位整型
#[derive(Debug, Deserialize)]
struct MsgId {
  #[serde(rename = "$value")]
  body: i64,
}

// 图片消息媒体id，可以调用获取临时素材接口拉取数据。
#[derive(Debug, Deserialize)]
struct MediaId {
  #[serde(rename = "$value")]
  body: String,
}

// 图片链接（由系统生成）
#[derive(Debug, Deserialize)]
struct PicUrl {
  #[serde(rename = "$value")]
  body: String,
}

// 消息类型，图片为image
#[derive(Debug, Deserialize)]
struct MsgType {
  #[serde(rename = "$value")]
  body: String,
}

// 消息创建时间 （整型）
#[derive(Debug, Deserialize)]
struct CreateTime {
  #[serde(rename = "$value")]
  body: i64,
}

// 发送方帐号（一个OpenID）
#[derive(Debug, Deserialize)]
struct FromUserName {
  #[serde(rename = "$value")]
  body: String,
}

// 开发者微信号
#[derive(Debug, Deserialize)]
struct ToUserName {
  #[serde(rename = "$value")]
  body: String,
}

#[derive(Debug, Deserialize)]
pub struct Xml {
  #[serde(rename = "ToUserName")]
  user_name: ToUserName,
  #[serde(rename = "FromUserName")]
  from_user_name: FromUserName,
  #[serde(rename = "CreateTime")]
  create_time: CreateTime,
  #[serde(rename = "MsgType")]
  msg_type: MsgType,
  #[serde(rename = "PicUrl")]
  pic_url: PicUrl,
  #[serde(rename = "MediaId")]
  media_id: MediaId,
  #[serde(rename = "MsgId")]
  msg_id: MsgId,
}

impl Xml {
    pub fn pic_url(&mut self) -> &str {
        self.pic_url.body.as_str()
    }
}

#[test]
fn test1() {
  use serde_xml_rs::from_reader;
  let xml: Xml = from_reader(
    r#"<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1348831860</CreateTime>
  <MsgType><![CDATA[image]]></MsgType>
  <PicUrl><![CDATA[this is a url]]></PicUrl>
  <MediaId><![CDATA[media_id]]></MediaId>
  <MsgId>1234567890123456</MsgId>
</xml>"#
      .as_bytes(),
  )
  .unwrap();
  println!("{:?}", xml);
}
