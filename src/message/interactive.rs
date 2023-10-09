use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;



/// 交互模块
/// https://open.feishu.cn/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN
/// 卡片提供 4 种交互控件（button，selectMenu，overflow，datePicker），你可以通过 actions 字段添加交互元素，实现交互功能。
/// 卡片交互 有效期为30天 ，超过有效期的卡片不支持交互，用户点击后将toast提示"该卡片发送时间已超过30天，无法进行互动"。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub tag: String,
    pub actions: Vec<ActionModule>,
    pub layout: Option<ActionLayout>,
}

/// 交互元素布局，窄版样式默认纵向排列。"bisected", "trisection", "flow"
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum ActionLayout {
    #[serde(rename = "bisected")]
    Bisected,
    #[serde(rename = "trisection")]
    Trisection,
    #[serde(rename = "flow")]
    Flow,
}


/// 消息卡片 可内嵌的交互元素 card_link
/// https://open.feishu.cn/document/ukTMukTMukTM/uYDN1UjL2QTN14iN0UTN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ActionCardLink {
    /// 默认的链接地址
    pub url: String,
    /// PC 端的链接地址
    pub pc_url: Option<String>,
    /// iOS 端的链接地址
    pub ios_url: Option<String>,
    /// Android 端的链接地址
    pub android_url: Option<String>,
}

/// 消息卡片 可内嵌的交互元素 confirm
/// https://open.feishu.cn/document/ukTMukTMukTM/ukzNwUjL5cDM14SO3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ActionConfirm {
    /// 弹框标题
    pub title: Text,
    /// 弹框内容
    pub text: Text,
}

/// 消息卡片 可内嵌的交互元素 option
/// https://open.feishu.cn/document/ukTMukTMukTM/ugzNwUjL4cDM14CO3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ActionOption {
    /// 选项显示内容，非待选人员时必填
    pub text: Option<Text>,
    /// 选项选中后返回业务方的数据，与url或multi_url必填其中一个
    pub value: Option<String>,
    /// *仅支持overflow，跳转指定链接，和multi_url字段互斥
    pub url: Option<String>,
    /// *仅支持overflow，跳转对应链接，和url字段互斥
    pub multi_url: Option<ActionUrl>,
}

/// 消息卡片 可内嵌的交互元素
/// https://open.feishu.cn/document/ukTMukTMukTM/uczNwUjL3cDM14yN3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ActionUrl {
    /// 默认跳转链接，参考注意事项-2
    pub url: String,
    /// 安卓端跳转链接
    pub android_url: String,
    /// ios端跳转链接
    pub ios_url: String,
    /// pc端跳转链接
    pub pc_url: String,
}


/// 消息卡片 交互模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
#[serde(tag = "tag")]
pub enum ActionModule {
    #[serde(rename = "button")]
    Button(Button),
    #[serde(rename = "select_static")]
    SelectStatic(SelectMenu),
    #[serde(rename = "select_person")]
    SelectPerson(SelectMenu),
    #[serde(rename = "overflow")]
    OverFlow(OverFlow),
    #[serde(rename = "date_picker")]
    DatePicker(DatePicker),
    #[serde(rename = "picker_time")]
    PickerTime(DatePicker),
    #[serde(rename = "picker_datetime")]
    PickerDatetime(DatePicker),
}


/// 消息卡片 交互组件 datePicker
/// https://open.feishu.cn/document/ukTMukTMukTM/uQzNwUjL0cDM14CN3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct DatePicker {
    pub initial_date: Option<String>,
    pub initial_time: Option<String>,
    pub initial_datetime: Option<String>,
    pub placeholder: Option<Text>,
    pub value: Option<HashMap<String, Value>>,
    pub confirm: Option<ActionConfirm>,
}

/// 消息卡片 交互组件 overflow
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzNwUjLzcDM14yM3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct OverFlow {
    pub options: Vec<ActionOption>,
    pub value: Option<HashMap<String, Value>>,
    pub confirm: Option<ActionConfirm>,
}

/// 消息卡片 交互组件 selectMenu
/// https://open.feishu.cn/document/ukTMukTMukTM/uIzNwUjLycDM14iM3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct SelectMenu {
    pub placeholder: Option<Text>,
    pub initial_option: Option<String>,
    pub options: Option<Vec<ActionOption>>,
    pub value: Option<HashMap<String, Value>>,
    pub confirm: Option<ActionConfirm>,
}


/// 消息卡片 交互组件 button
/// https://open.feishu.cn/document/ukTMukTMukTM/uEzNwUjLxcDM14SM3ATN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Button {
    pub text: Text,
    pub url: Option<String>,
    pub multi_url: Option<ActionUrl>,
    #[serde(rename = "type")]
    pub type_: Option<ButtonType>,
    pub value: Option<HashMap<String, Value>>,
    pub confirm: Option<ActionConfirm>,
}

/// 消息卡片 交互组件 button type
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum ButtonType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "danger")]
    Danger,
}

/// 消息卡片 正文内容
pub type Element = ContentModule;

/// 消息卡片 多语言正文内容
pub type I18nElements = HashMap<Language, ContentModule>;

/// 消息卡片 功能属性
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Config {
    /// 是否允许卡片被转发。
    pub enable_forward: Option<bool>,
    /// 是否为共享卡片。
    pub update_multi: Option<bool>,
}


/// 消息卡片 可内嵌的非交互元素 field
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Field {
    pub is_short: bool,
    pub text: Text,
}

/// 消息卡片 可内嵌的非交互元素 text
#[derive(Debug, Clone, Deserialize, Serialize, )]
#[serde(tag = "tag")]
pub enum Text {
    #[serde(rename = "plain_text")]
    PlainText(TextContent),
    #[serde(rename = "lark_md")]
    LarkMd(TextContent),
}

#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct TextContent {
    pub content: String,
    pub lines: Option<u32>,
}

/// 消息卡片 可内嵌的非交互元素 image
#[derive(Debug, Clone, Deserialize, Serialize, )]
#[serde(tag = "tag")]
pub enum Image {
    #[serde(rename = "img")]
    Img(ImageContent),
}

#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ImageContent {
    pub img_key: String,
    pub alt: Text,
    pub preview: Option<bool>,
}


#[derive(Debug, Clone, Deserialize, Serialize, )]
#[serde(untagged)]
pub enum Extra {
    Image(Image),
    ActionModule(ActionModule),
}

/// 消息卡片 卡片标题
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Header {
    /// 配置卡片标题内容 text 对象（仅支持"plain_text")
    pub title: HeaderTitle,
    /// 控制标题背景颜色，取值参考注意事项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<HeaderTemplate>,
}

pub type HeaderTitle = Text;


/// 消息卡片 卡片标题 卡片标题的主题色
///
/// 最佳实践
/// 彩色标题适合在群聊中使用，对于需高亮提醒的卡片信息，可将标题配置为应用的品牌色或表达状态的语义色，增强信息的视觉锚点。
/// 在单聊中，建议根据卡片的状态不同，配置不同的语义颜色标题
/// 绿色（Green）代表完成/成功
/// 橙色（Orange）代表警告/警示
/// 红色（Red）代表错误/异常
/// 灰色（Grey）代表失效
/// 不建议在单聊中，每张卡片无差别地使用同一种颜色的标题。这样既无法起到强调作用，也可能引起阅读者的视觉焦虑。
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum HeaderTemplate {
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "wathet")]
    Wathet,
    #[serde(rename = "turquoise")]
    Turquoise,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "carmine")]
    Carmine,
    #[serde(rename = "violet")]
    Violet,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "indigo")]
    Indigo,
    #[serde(rename = "grey")]
    Grey,
}


/// 消息卡片 布局 多列布局
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ColumnSet {
    pub tag: String,
    pub columns: Option<Vec<Column>>,
    pub flex_mode: Option<String>,
    pub background_style: Option<String>,
    pub horizontal_spacing: Option<String>,
    /// 设置点击布局容器时，响应的交互配置。当前仅支持跳转交互。如果布局容器内有交互组件，则优先响应交互组件定义的交互。
    pub action: Option<ColumnSetAction>,
}


/// 消息卡片 布局 列
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Column {
    pub tag: String,
    pub elements: Option<ColumnElements>,
    pub width: Option<String>,
    pub weight: Option<u8>,
    pub vertical_align: Option<ColumnVerticalAlign>,
}

/// 消息卡片 布局 列 卡片元素
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum ColumnElements {
    Elements(Vec<ContentModule>),
    ColumnSet(Vec<ColumnSet>),
}


/// 消息卡片 布局 多列布局 交互配置
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct ColumnSetAction {
    pub multi_url: ActionUrl,
}

/// 消息卡片 布局 列 垂直对齐
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum ColumnVerticalAlign {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "bottom")]
    Bottom,
}


/// 消息卡片 正文内容
#[derive(Debug, Clone, Deserialize, Serialize, )]
#[serde(tag = "tag")]
pub enum ContentModule {
    #[serde(rename = "div")]
    Div(Div),
    #[serde(rename = "markdown")]
    Markdown(Markdown),
    #[serde(rename = "hr")]
    Hr(Hr),
    #[serde(rename = "img")]
    Img(Img),
    #[serde(rename = "note")]
    Note(Note),
}


/// 消息卡片 内容模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Div {
    pub text: Text,
    pub fields: Option<Vec<Field>>,
    pub extra: Option<Extra>,
}

/// 消息卡片 Markdown模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Markdown {
    pub content: String,
    pub text_align: Option<TextAlign>,
    pub href: Option<Href>,
}

/// 消息卡片 Markdown模块 text_align
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// 消息卡片 Markdown模块 href
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Href {
    #[serde(rename = "urlVal")]
    pub url_val: ActionUrl,
}

/// 消息卡片 分割线模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Hr {}

/// 消息卡片 图片模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Img {
    pub img_key: String,
    pub alt: Text,
    pub title: Option<Text>,
    pub custom_width: Option<u16>,
    pub compact_width: Option<bool>,
    pub mode: Option<ImgMode>,
    pub preview: Option<bool>,
}

/// 消息卡片 图片模块 mode
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum ImgMode {
    #[serde(rename = "fit_horizontal")]
    FitHorizontal,
    #[serde(rename = "crop_center")]
    CropCenter,
}

/// 消息卡片 分割线模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct Note {
    pub elements: NoteElements,
}

/// 消息卡片 备注模块
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum NoteElements {
    Text(Vec<Text>),
    Image(Vec<Image>),
}


/// AppLink 协议 就是一个 URL 协议。AppLink 协议可以用于打开飞书或者其中的一个功能。
/// https://open.feishu.cn/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct AppLink {
    pub scheme: Scheme,
    pub host: Host,
    pub path: Path,
    pub query: HashMap<String, String>,
}

// impl Display for AppLink {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let scheme = &self.scheme;
//         let host = &self.host;
//         let path = &self.path;
//         let query = &self.query;
//         todo!();
//         f.write_str(format!("{:?}://{:?}{:?}?{:?}", scheme, host, path, query).as_str())
//     }
// }

/// 协议
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum Scheme {
    #[serde(rename = "https")]
    Https
}

/// 域名
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum Host {
    #[serde(rename = "applink.feishu.cn")]
    AppLinkFeishuCn,
}

/// 路径，不同路径打开不同功能
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum Path {
    /// 打开飞书
    #[serde(rename = "/client/op/open")]
    Feishu,
    /// 打开扫一扫
    #[serde(rename = "/client/qrcode/main")]
    QrCode,
    /// 打开小程序
    #[serde(rename = "/client/mini_program/open")]
    MiniProgram,
    /// 打开网页应用
    #[serde(rename = "/client/web_app/open")]
    WebApp,
    /// 打开聊天页面
    #[serde(rename = "/client/chat/open")]
    Chat,
    /// 打开日历
    #[serde(rename = "/client/calendar/open")]
    Calendar,
    /// 打开日历（支持定义视图和日期）
    #[serde(rename = "/client/calendar/view")]
    CalendarView,
    /// 打开日程创建页
    #[serde(rename = "/client/calendar/event/create")]
    CalendarEventCreate,
    /// 打开第三方日历账户管理页
    #[serde(rename = "/client/calendar/account")]
    CalendarAccount,
    /// 打开任务
    #[serde(rename = "/client/todo/open")]
    Todo,
    /// 创建任务
    #[serde(rename = "/client/todo/create")]
    TodoCreate,
    /// 打开任务详情页
    #[serde(rename = "/client/todo/detail")]
    TodoDetail,
    /// 打开任务Tab页
    #[serde(rename = "/client/todo/view")]
    TodoView,
    /// 打开云文档
    #[serde(rename = "/client/docs/open")]
    Docs,
    /// 打开机器人会话
    #[serde(rename = "/client/bot/open")]
    Bot,
    /// 打开SSO登录页
    #[serde(rename = "/client/passport/sso_login")]
    PassportSsoLogin,
    /// 打开PC端内web-view访问指定URL
    #[serde(rename = "/client/web_url/open")]
    WebUrl,
}

/// 打开小程序 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryMiniProgram {
    /// 小程序 appId(可从「开发者后台-凭证与基础信息」获取)
    #[serde(rename = "appId")]
    pub app_id: String,
    /// PC小程序启动模式 PC 建议填写
    pub mode: Option<QueryMiniProgramMode>,
    /// 自定义独立窗口高度（仅当mode为window时生效）
    pub height: Option<u32>,
    /// 自定义独立窗口宽度（仅当mode为window时生效）
    pub width: Option<u32>,
    /// 是否重新加载指定页面。该参数仅当applink中传入path参数时才会生效
    pub relaunch: Option<bool>,
    /// 需要跳转的页面路径，路径后可以带参数。也可以使用 path_android、path_ios、path_pc 参数对不同的客户端指定不同的path
    pub path: Option<MiniProgramPath>,
    /// 同 path 参数，Android 端会优先使用该参数，如果该参数不存在，则会使用 path 参数
    pub path_android: Option<MiniProgramPath>,
    /// 同 path 参数，iOS 端会优先使用该参数，如果该参数不存在，则会使用 path 参数
    pub path_ios: Option<MiniProgramPath>,
    /// 同 path 参数，PC 端会优先使用该参数，如果该参数不存在，则会使用 path 参数
    pub path_pc: Option<MiniProgramPath>,
    /// 指定 AppLink 协议能够兼容的最小飞书版本，使用三位版本号 x.y.z。如果当前飞书版本号小于min_lk_ver，打开该 AppLink 会显示为兼容页面
    pub min_lk_ver: Option<String>,
}

pub type MiniProgramPath = String;

/// PC小程序启动模式
/// 未填写的情况下，默认会优先使用sidebar-semi打开，不支持sidebar-semi模式的情况下会使用window-semi模式打开
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum QueryMiniProgramMode {
    /// 聊天的侧边栏打开
    #[serde(rename = "sidebar-semi")]
    SidebarSemi,
    /// 工作台中打开
    #[serde(rename = "appCenter")]
    AppCenter,
    /// 独立大窗口打开
    #[serde(rename = "window")]
    Window,
    /// 独立小窗口打开，飞书3.33版本开始支持此模式
    #[serde(rename = "window-semi")]
    WindowSemi,
}

/// 打开网页应用 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryWebApp {
    /// H5应用的 appId(可从「开发者后台-凭证与基础信息」获取)
    #[serde(rename = "appId")]
    pub app_id: String,
    /// 打开H5应用的容器模式
    pub mode: Option<QueryWebAppMode>,
    /// 自定义独立窗口高度（仅当mode为window时生效）
    pub height: Option<u32>,
    /// 自定义独立窗口宽度（仅当mode为window时生效）
    pub width: Option<u32>,
    /// 访问H5应用的具体某个页面，path参数将替换H5应用URL的path部分。如果需要携带参数，将预期的H5应用URL的query作为applink的query即可，
    pub path: Option<WebAppPath>,
    /// 同 path 参数，Android 端会优先使用该参数，如果该参数不存在，则会使用 path 参数。
    pub path_android: Option<WebAppPath>,
    /// 同 path 参数，iOS 端会优先使用该参数，如果该参数不存在，则会使用 path 参数
    pub path_ios: Option<WebAppPath>,
    /// 同 path 参数，PC 端会优先使用该参数，如果该参数不存在，则会使用 path 参数
    pub path_pc: Option<WebAppPath>,
    /// 访问H5应用的具体某个页面，针对网页path中包含#或?字符时，可使用该参数，而不使用path参数。需要填写H5应用某个具体页面的完整URL（协议名scheme和域名domain应当与开发者后台配置的应用首页相匹配），并进行URL encode后使用
    pub lk_target_url: Option<String>,
    /// 如果网页应用当前所处的页面路径和 applink 要打开的页面路径相同时：true：重新加载页面 false：保留原页面状态
    pub reload: Option<bool>,
}

pub type WebAppPath = String;
pub type WebAppQuery = HashMap<String, String>;

/// 打开H5应用的容器模式
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum QueryWebAppMode {
    /// 在工作台打开，3.20版本开始支持（缺省值）
    #[serde(rename = "appCenter")]
    AppCenter,
    /// 在独立窗口打开，3.20版本开始支持
    #[serde(rename = "window")]
    Window,
    /// 在侧边栏打开，3.40版本开始支持
    #[serde(rename = "sidebar")]
    SideBar,
    /// 在独立窗口以小屏形式打开，5.10版本开始支持
    #[serde(rename = "window-semi")]
    WindowSemi,
}

/// 打开聊天应用 参数 openID与openChatId 仅能填写其中一个参数。
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryChat {
    /// 用户 openId，获取方式可以参考文档：如何获得 User ID、Open ID 和 Union ID？
    #[serde(rename = "openId")]
    pub open_id: Option<String>,
    /// 会话ID，包括单聊会话和群聊会话。是以 'oc' 开头的字段,获取方式可参考文档 群ID说明；示例值： oc_41e7bdf4877cfc316136f4ccf6c32613
    #[serde(rename = "openChatId")]
    pub open_chat_id: Option<String>,
}

/// 打开日历 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryCalendarView {
    /// 视图类型
    #[serde(rename = "type")]
    pub type_: Option<QueryCalendarViewType>,
    /// 日期，{unixTime}格式
    pub date: Option<u64>,
}

/// 视图类型
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum QueryCalendarViewType {
    /// 日视图
    #[serde(rename = "day")]
    Day,
    /// 三日视图，仅移动端支持
    #[serde(rename = "three_day")]
    ThreeDay,
    /// 周视图，仅PC端支持
    #[serde(rename = "week")]
    Week,
    /// 月视图
    #[serde(rename = "month")]
    Month,
    /// 会议室视图，仅PC端支持
    #[serde(rename = "meeting")]
    Meeting,
    /// 列表视图，仅移动端支持
    #[serde(rename = "list")]
    List,
}

/// 打开日程创建页 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryCalendarEventCreate {
    /// 开始日期，{unixTime}格式
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// 结束日期，{unixTime}格式
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// 日程主题，中文可使用encodeURIComponent方法生成
    pub summary: Option<String>,
}

/// 打开任务详情 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryTodoDetail {
    /// 全局唯一的taskId（global unique ID）,通过飞书任务的 OpenAPI 获取
    pub guid: String,
    /// 默认在im场景下，打开任务详情页面；mode=app, 在任务tab中打开详情页面
    pub mode: Option<String>,
}

/// 打开任务tab页 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryTodoView {
    pub tab: QueryTodoViewTab,
}

/// 任务tab页
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum QueryTodoViewTab {
    /// 进行中
    #[serde(rename = "all")]
    All,
    /// 由我处理
    #[serde(rename = "assign_to_me")]
    AssignToMe,
    /// 我分配的
    #[serde(rename = "assign_from_me")]
    AssignFromMe,
    /// 我关注的
    #[serde(rename = "followed")]
    Followed,
    /// 已完成
    #[serde(rename = "completed")]
    Completed,
}

/// 打开云文档 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryDocs {
    /// 要打开的云文档URL
    #[serde(rename = "appId")]
    pub url: String,
}

/// 打开机器人会话 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryBot {
    /// 机器人的appId
    #[serde(rename = "appId")]
    pub app_id: String,
}

/// 打开SSO登录页 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryPassportSsoLogin {
    /// 租户的域名，填写的是租户在admin后台配置的租户域名信息。当在admin后台改动租户的域名时，需要同步修改applink该参数值
    pub sso_domain: String,
    /// 租户名，用于在切换租户时，客户端展示即将登录到的租户名称，一般填写公司名即可
    pub tenant_name: String,
}

/// 打开PC端内web-view访问指定URL 参数
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub struct QueryWebUrl {
    /// 指定需要在客户端内打开的具体链接，需要执行encodeURIComponent，4.2+版本支持lark协议
    pub url: String,
    /// PC端打开的容器模式
    pub mode: QueryWebUrlMode,
    /// PC端自定义独立窗口高度
    pub height: Option<u32>,
    /// PC端自定义独立窗口宽度
    pub width: Option<u32>,

}

/// PC端打开的容器模式
#[derive(Debug, Clone, Deserialize, Serialize, )]
pub enum QueryWebUrlMode {
    /// 在侧边栏打开
    #[serde(rename = "sidebar-semi")]
    SidebarSemi,
    /// 在独立窗口打开
    #[serde(rename = "window")]
    Window,
}