use std::collections::HashMap;
use lazy_static::lazy_static;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
        ParseMode::{
            MarkdownV2
        },
    },
    utils::markdown::code_inline,
};

fn get_encoding_dict() -> HashMap<u8, String> {
    let mut dict: HashMap<u8, String> = HashMap::new();
    let default: [(u8, &str); 65] = [
        (65, "1145"), (66, "1154"), (67, "1514"), (68, "1415"), (69, "1541"),
        (70, "1451"), (71, "5114"), (72, "4115"), (73, "5141"), (74, "4151"),
        (75, "4511"), (76, "5411"), (77, "5541"), (78, "5514"), (79, "5154"),
        (80, "5451"), (81, "5145"), (82, "5415"), (83, "1554"), (84, "4551"),
        (85, "4515"), (86, "1545"), (87, "1455"), (88, "4155"), (89, "4415"),
        (90, "4451"), (97, "4541"), (98, "4145"), (99, "4154"), (100, "4514"),
        (101, "1445"), (102, "5441"), (103, "1454"), (104, "5414"), (105, "5144"),
        (106, "1544"), (107, "1114"), (108, "1115"), (109, "5551"), (110, "5554"),
        (111, "4441"), (112, "4445"), (113, "1151"), (114, "1141"), (115, "5515"),
        (116, "5545"), (117, "4454"), (118, "4414"), (119, "1511"), (120, "1411"),
        (121, "4544"), (122, "4144"), (48, "5455"), (49, "5155"), (50, "1111"),
        (51, "4444"), (52, "5555"), (53, "1155"), (54, "1144"), (55, "5511"),
        (56, "5544"), (57, "4455"), (43, "4411"), (47, "5115"), (61, "4114")
    ];
    for (key, value) in default.iter() {
        dict.insert(
            key.clone(),
            value.to_string(),
        );
    }
    dict
}

lazy_static! {
    static ref ENCODING_DICT: HashMap<u8, String> = get_encoding_dict();
}


#[allow(deprecated)]
fn base114514_encode(
    string: &String,
) -> String {
    let base64encoded_string = base64::encode(string.as_bytes());
    let mut encoded_string = String::new();
    for char in base64encoded_string.as_bytes().iter().enumerate() {
        encoded_string.push_str(ENCODING_DICT.get(char.1).unwrap());
    }
    encoded_string
}

pub fn function(_: &Bot, q: &InlineQuery) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        "base114514".to_string(), // id
        "Base114514 编码", // title
        InputMessageContent::Text(
            InputMessageContentText::new({
                code_inline(base114514_encode(&q.query).as_str())
            }).parse_mode(MarkdownV2)
        ),
    ).description(
        "来自下北泽的恶臭编码".to_string(),
    )
}