use teloxide::{
    prelude::*,
    Bot,
    types::InlineQueryResult
};

mod config;
mod inline_handler;

use config::CONFIG;
use inline_handler::{
    wrap,
    empty,
    fabing,
    base114514,
    base114514_decode,
    american_stone,
    gengshuang
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("正在启动 bot...");

    let bot = Bot::new(CONFIG.bot.token.clone());

    // inline 查询
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        move |bot: Bot, q: InlineQuery| async move {
            if q.query.is_empty() {
                bot.answer_inline_query(&q.id, vec![
                    wrap::handler(empty::function, &bot, &q),
                ]).send().await?;
                return respond(());
            }

            // 添加处理器
            let mut results = vec![
                wrap::handler(fabing::function, &bot, &q),
                wrap::handler(base114514::function, &bot, &q),
                wrap::handler(american_stone::function, &bot, &q),
                InlineQueryResult::Article(gengshuang::function(
                    &bot, &q, CONFIG.modules.gengshuang_api.clone()
                ).await),
            ];

            // 添加条件处理器
            if base114514_decode::check(&q) {
                results.push(
                    wrap::handler(base114514_decode::function, &bot, &q)
                );
            }

            let resp = bot.
                answer_inline_query(&q.id, results).
                cache_time(CONFIG.bot.cache_time)
                .send().await;
            if let Err(err) = resp {
                log::error!("发送 inline 查询结果时出错：{}", err);
            }
            respond(())
        }
    ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}