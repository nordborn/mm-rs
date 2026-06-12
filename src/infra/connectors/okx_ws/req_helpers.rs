use url::Url;

use super::OkxWs;

impl OkxWs {
    pub fn public_subscribe_url() -> Url {
        Url::parse("wss://ws.okx.com:8443/ws/v5/public").unwrap()
    }
}
