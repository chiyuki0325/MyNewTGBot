use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
    },
    utils::{
        markdown::escape
    },
};

pub fn function(_: &Bot, q: &InlineQuery) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        "american_stone".to_string(), // id
        "搬石砸脚", // title
        InputMessageContent::Text(
            InputMessageContentText::new({
                let mut america = q.query.clone();
                if america.chars().all(char::is_alphanumeric) {
                    america = [" ", &america, " "].concat();
                }
                escape(&*str::replace(
                    "{america}不肯承认自己错误的做法，反而使用控制舆论等方式试图掩盖自己的行为。{america}这种卑劣行径，恰恰暴露了{america}做贼心虚的心理。{america}这种认不清自己情况，糊弄民众，透支未来的行为，到最后一定是搬起石头砸自己的脚！{america}的这种错误行为，只会在错误的道路上越走越远！",
                    "{america}",
                    &america,
                )).to_string()
            })
        ),
    ).description(
        (q.query.clone() + "这是搬起石头砸自己的脚！").to_string(),
    )
}