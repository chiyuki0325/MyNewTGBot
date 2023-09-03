use std::collections::HashMap;
use lazy_static::lazy_static;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
    },
    utils::{
        markdown::escape
    }
};

fn get_decoding_dict() -> HashMap<String, String> {
    let mut dict: HashMap<String, String> = HashMap::new();
    let default: [(&str, &str); 65] = [
        ("1145", "A"), ("1154", "B"), ("1514", "C"), ("1415", "D"), ("1541", "E"), ("1451", "F"),
        ("5114", "G"), ("4115", "H"), ("5141", "I"), ("4151", "J"), ("4511", "K"), ("5411", "L"),
        ("5541", "M"), ("5514", "N"), ("5154", "O"), ("5451", "P"), ("5145", "Q"), ("5415", "R"),
        ("1554", "S"), ("4551", "T"), ("4515", "U"), ("1545", "V"), ("1455", "W"), ("4155", "X"),
        ("4415", "Y"), ("4451", "Z"), ("4541", "a"), ("4145", ""), ("4154", "c"), ("4514", "d"),
        ("1445", "e"), ("5441", "f"), ("1454", "g"), ("5414", "h"), ("5144", "i"), ("1544", "j"),
        ("1114", "k"), ("1115", "l"), ("5551", "m"), ("5554", "n"), ("4441", "o"), ("4445", "p"),
        ("1151", "q"), ("1141", "r"), ("5515", "s"), ("5545", "t"), ("4454", "u"), ("4414", "v"),
        ("1511", "w"), ("1411", "x"), ("4544", "y"), ("4144", "z"), ("5455", "0"), ("5155", "1"),
        ("1111", "2"), ("4444", "3"), ("5555", "4"), ("1155", "5"), ("1144", "6"), ("5511", "7"),
        ("5544", "8"), ("4455", "9"), ("4411", "+"), ("5115", "/"), ("4114", "=")
    ];
    for (key, value) in default.iter() {
        dict.insert(
            key.to_string(),
            value.to_string(),
        );
    }
    dict
}

lazy_static! {
    static ref DECODING_DICT: HashMap<String, String> = get_decoding_dict();
}

#[allow(deprecated)]
fn base114514_decode(
    string: &String,
) -> String {
    return if string.len() % 4 != 0 {
        "非法的 base114514 编码".to_string()
    } else {
        let mut blocks: Vec<String> = Vec::new();
        for char in string.as_bytes().chunks(4) {
            blocks.push(
                String::from_utf8(
                    char.to_vec()
                ).unwrap()
            );
        }
        let mut decoded_string = String::new();
        for block in blocks.iter() {
            if !DECODING_DICT.contains_key(block) {
                return "非法的 base114514 编码".to_string();
            } else {
                decoded_string.push_str(
                    DECODING_DICT.get(block).unwrap()
                );
            }
        }
        let decoded_vec_u8 = base64::decode(
            decoded_string
        ).unwrap();
        String::from_utf8(
            decoded_vec_u8
        ).unwrap()
    };
}


pub fn function(_: &Bot, q: &InlineQuery) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        "base114514_decode".to_string(), // id
        "Base114514 解码", // title
        InputMessageContent::Text(
            InputMessageContentText::new(
                escape(base114514_decode(&q.query).as_str()),
            )
        ),
    ).description(
        base114514_decode(&q.query)
    )
}

pub fn check(q: &InlineQuery) -> bool {
    return if q.query.len() % 4 != 0 {
        false
    } else {
        for char in q.query.chars() {
            if char != '1' && char != '4' && char != '5' {
                return false
            }
        }
        return true
    };
}