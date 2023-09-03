use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
    },
    utils::{
        markdown
    },
};


pub async fn function(_: &Bot, q: &InlineQuery, url: String) -> InlineQueryResultArticle {
    if q.query.contains('|') {
        InlineQueryResultArticle::new(
            "gengshuang".to_string(), // id
            "耿爽模拟器", // title
            InputMessageContent::Text(
                InputMessageContentText::new({
                    let mut parts = q.query.split('|');
                    let dickman = parts.next();
                    let what_dickman_did = parts.next();
                    match reqwest::get(
                        [
                            url.as_str(),
                            "?isnegative=1&name=",
                            dickman.unwrap(),
                            "&did=",
                            what_dickman_did.unwrap()
                        ].concat()
                    ).await {
                        Ok(response) => {
                            markdown::escape(&("耿爽: ".to_string() + &response.text().await.unwrap())).to_string()
                        },
                        Err(_) => "耿爽: 连接 API 失败。".to_string()
                    }
                })
            ),
        ).description(
            format!("中方严厉谴责{}的行为！", q.query.replace("|", ""))
        )
    } else {
        InlineQueryResultArticle::new(
            "gengshuang_help".to_string(), // id
            "耿爽模拟器 | 使用帮助", // title
            InputMessageContent::Text(
                InputMessageContentText::new(
                    "使用方法: 人物|行为"
                )
            ),
        ).description("人物|行为")
    }
}