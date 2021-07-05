use binance::api::*;
use binance::market::*;

fn get_crypto_price(crypto_name: &str) -> Result<f64, anyhow::Error> {
    let market: Market = Binance::new(None, None);

    match market.get_price(crypto_name) {
        Ok(symbol_price) => Ok(symbol_price.price),
        Err(err) => Err(anyhow::anyhow!(
            "Error, while parsing crypto price: {:?}",
            err
        )),
    }
}

fn main() {
    println!("{:?}", get_crypto_price("NEARUDT"));
}
