use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult,
        InlineQueryResultArticle,
    },
};

pub fn handler(
    handler_fn: fn(&Bot, &InlineQuery) -> InlineQueryResultArticle,
    bot: &Bot,
    q: &InlineQuery
) -> InlineQueryResult {
    InlineQueryResult::Article(
        handler_fn(bot, q)
    )
}