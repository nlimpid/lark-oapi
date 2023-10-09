use serde::{Deserialize, Serialize};
use crate::message::interactive::{Config, Element, Header, I18nElements};

/// the definition of MsgType  is come from https://open.feishu.cn/document/server-docs/im-v1/message-content-description/create_json
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "msg_type", content = "content")]
pub enum MsgType {
    /// 文本
    #[serde(rename = "text")]
    Text(MsgText),
    /// 消息卡片
    #[serde(rename = "interactive")]
    Interactive(MsgInteractive),
    // /// 富文本
    // #[serde(rename = "post")]
    // Post(FeishuPostMsg),
    // /// 图片
    // #[serde(rename = "image")]
    // Image(FeishuImageMsg),
    //
    // /// 分享群名片
    // #[serde(rename = "share_chat")]
    // ShareChat(FeishuShareChatMsg),
    // /// 分享个人名片
    // #[serde(rename = "share_user")]
    // ShareUser(FeishuShareUserMsg),
    // /// 语音
    // #[serde(rename = "audio")]
    // Audio(FeishuAudioMsg),
    // /// 视频
    // #[serde(rename = "media")]
    // Media(FeishuMediaMsg),
    // /// 文件
    // #[serde(rename = "file")]
    // File(FeishuFileMsg),
    // /// 表情包
    // #[serde(rename = "sticker")]
    // Sticker(FeishuStickerMsg),
}

/// 文本
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MsgText {
    pub text: String,
}


/// 消息卡片, https://open.feishu.cn/document/common-capabilities/message-card/introduction-of-message-cards
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MsgInteractive {
    /// 用于描述卡片的功能属性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// 用于配置卡片标题内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    /// 用于定义卡片正文内容，和i18n_elements至少必填其中1个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<Element>>,
    /// 为卡片的正文部分定义多语言内容，和elements至少必填其中1个
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_elements: Option<I18nElements>,
}


/// 语言代码
#[derive(Debug, Clone, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Language {
    #[serde(rename = "zh_cn")]
    ZhCn,
    #[serde(rename = "en_us")]
    EnUs,
}