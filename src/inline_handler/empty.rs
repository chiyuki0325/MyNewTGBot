use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
    },
};
use teloxide::types::InputMessageContentText;

pub fn function(_: &Bot, _: &InlineQuery) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        "empty".to_string(), // id
        "内容为空", // title
        InputMessageContent::Text(
            InputMessageContentText::new(
                "内容为空",
            )
        ),
    ).description(
        "请先输入内容"
    )
}