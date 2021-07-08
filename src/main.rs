use teloxide::prelude::*;

use binance::api::*;
use binance::market::*;

// async fn get_updates(url: &str) {}

async fn get_crypto_price(crypto_name: &'static str) -> Result<f64, anyhow::Error> {
    let market: Market = Binance::new(None, None);

    tokio::task::spawn_blocking(move || match market.get_price(crypto_name) {
        Ok(symbol_price) => Ok(symbol_price.price),
        Err(err) => Err(anyhow::anyhow!(
            "Error, while parsing crypto price: {:?}",
            err
        )),
    })
    .await?
}

#[tokio::main]
async fn main() {
    // teloxide::enable_logging!();
    // log::info!("Starting binance-notificator-bot...");
    // let bot = Bot::from_env().auto_send();

    // teloxide::types::InlineKeyboardButton::text("Будильник на понижение");

    match get_crypto_price("NEARUSDT").await {
        Ok(res) => println!("{}", res),
        Err(err) => log::info!("{}", err),
    }
}
