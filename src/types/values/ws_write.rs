use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

pub type WsWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
