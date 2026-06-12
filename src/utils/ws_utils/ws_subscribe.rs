use anyhow::Result;
use bytes::Bytes;
use futures_util::{
    SinkExt,   // write.send()
    StreamExt, // stream.split()
};
use tokio::time::{Duration, sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

// https://github.com/snapview/tokio-tungstenite/blob/master/examples/client.rs
// Реализует дефолтную обраюботку веблокетов с передачей обработки данных сообщений функции
// `process_message_data`
// Функция блокирующая, в tokio::spawn(async move {ws_subscribe(...).await})
pub async fn ws_subscribe(
    topic_url: Url,
    on_connect_send: Option<&str>,
    process_message_data: impl AsyncFn(Bytes) -> Result<()> + Send + 'static,
) {
    let ctx = format!("{topic_url}: subscribe");
    log::info!("{ctx}");
    loop {
        match connect_async(topic_url.as_str()).await {
            Ok((stream, _)) => {
                log::info!("{ctx}: connected");
                let (mut write, mut read) = stream.split();

                if let Some(txt) = on_connect_send {
                    let _ = write.send(Message::Text(txt.into())).await;
                }

                #[cfg(feature = "hotpath")]
                let mut read = hotpath::stream!(read, log = true, label = topic_url);

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Ping(data)) => {
                            let _ = write.send(Message::Pong(data)).await;
                        }
                        Ok(msg) => {
                            let _ = process_message_data(msg.into_data())
                                .await
                                .map_err(|e| log::warn!("{ctx}: {e:#?}"));
                        }
                        Err(e) => {
                            log::error!("{ctx}: error reading message: {e:#?}");
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("{ctx}: error connecting to server: {:#?}, repeat", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
