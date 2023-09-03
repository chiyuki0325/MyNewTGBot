use teloxide::{
    prelude::*,
    types::{
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText
    },
    utils::{
        markdown::escape
    }
};

pub fn function(_: &Bot, q: &InlineQuery) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        "fabing".to_string(), // id
        "发病", // title
        InputMessageContent::Text(
            InputMessageContentText::new({
                let mut dickman = q.query.clone();
                if dickman.chars().all(char::is_alphanumeric) {
                    dickman = [" ", &dickman, " "].concat();
                }
                escape(&*str::replace(
                    "{dickman}……🤤嘿嘿………🤤……好可爱……嘿嘿……{dickman}🤤……{dickman}……我的🤤……嘿嘿……🤤………亲爱的……赶紧让我抱一抱……啊啊啊{dickman}软软的脸蛋🤤还有软软的小手手……🤤…{dickman}……不会有人来伤害你的…🤤你就让我保护你吧嘿嘿嘿嘿嘿嘿嘿嘿🤤……太可爱了……🤤……美丽可爱的{dickman}……像珍珠一样……🤤嘿嘿……{dickman}……🤤嘿嘿……🤤……好想一口吞掉……🤤……但是舍不得啊……我的{dickman}🤤……嘿嘿……🤤我的宝贝……我最可爱的{dickman}……🤤没有{dickman}……我就要死掉了呢……🤤我的……🤤嘿嘿……可爱的{dickman}……嘿嘿🤤……可爱的{dickman}……嘿嘿🤤🤤……可爱的{dickman}……🤤……嘿嘿🤤……可爱的{dickman}…（吸）身上的味道……好好闻～🤤…嘿嘿🤤……摸摸～……可爱的{dickman}……再贴近我一点嘛……（蹭蹭）嘿嘿🤤……可爱的{dickman}……嘿嘿🤤……～亲一口～……可爱的{dickman}……嘿嘿🤤……抱抱你～可爱的{dickman}～（舔）喜欢～真的好喜欢～……（蹭蹭）脑袋要融化了呢～已经……除了{dickman}以外～什么都不会想了呢～🤤嘿嘿🤤……可爱的{dickman}……嘿嘿🤤……可爱的{dickman}……我的～……嘿嘿🤤……",
                    "{dickman}",
                    &dickman,
                )).to_string()
            })
        ),
    ).description(
        (q.query.clone() + "……🤤嘿嘿………🤤……好可爱……嘿嘿……").to_string(),
    )
}