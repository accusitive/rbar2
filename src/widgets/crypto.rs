use crate::{TXType, Widget, WidgetType};

pub struct Crypto(pub TXType);
pub struct CryptoState {
    amount: f32,
    ticker: String,
    fiat: String,
}
impl Default for CryptoState {
    fn default() -> Self {
        let amount = dotenv::var("crypto_amount")
            .unwrap_or("1".to_string())
            .parse()
            .expect("crypto_amount env variable is malformed.");

        let ticker = dotenv::var("crypto_ticker").unwrap_or("btc".to_string());
        let fiat = dotenv::var("crypto_fiat").unwrap_or("usd".to_string());

        Self {
            amount,
            ticker,
            fiat,
        }
    }
}
impl<'a> Widget<'a> for Crypto {
    type WState = CryptoState;

    fn update(state: &mut Self::WState) -> Option<String> {
        let r = ureq::get(&format!(
            "https://{}.rate.sx/{}{}",
            state.fiat, state.amount, state.ticker
        ))
        .call()
        .unwrap();
        match r.into_string() {
            Ok(s) => Some(format!("💱 {}", s.split_at(s.len() - 9).0)),
            Err(e) => {
                eprintln!("e {}", e);
                None
            }
        }
        // let s = r.into_string().ok()?;
    }

    fn get_delta() -> u64 {
        60 * 1000
    }

    fn get_name() -> String {
        "Crypto".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }
    fn get_widget_type() -> WidgetType {
        WidgetType::Crypto
    }
}
