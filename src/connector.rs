use futures_util::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::WebSocketStream;
use tokio_native_tls::TlsStream;
use tokio::net::TcpStream;
use tungstenite::Message;

pub struct Connected {
    read: SplitStream<WebSocketStream<TlsStream<TcpStream>>>,
    write: SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>,
}

pub struct Connector {
    tls: tokio_native_tls::TlsConnector,
}

impl Connector {
    fn new(tls: tokio_native_tls::TlsConnector) -> Self {
        Self { tls }
    }

    pub fn builder() -> ConnectorBuilder {
        ConnectorBuilder::default()
    }

    pub async fn connect(&self, req: tungstenite::http::Request<()>) -> Connected {
        let uri = req.uri();

        let host = uri.host().unwrap();
        let port = uri.port().unwrap();
        let combo = format!("{host}:{port}");

        let stream = tokio::net::TcpStream::connect(&combo).await.unwrap();
        let stream = self.tls.connect(&combo, stream).await.unwrap();

        let (stream, _) = tokio_tungstenite::client_async(req, stream).await.expect("Failed to connect");

        let (write, read) = stream.split();

        Connected { write, read }
    }
}

#[derive(Default)]
pub struct ConnectorBuilder {
    insecure: bool,
}

impl ConnectorBuilder {
    pub fn insecure(self, value: bool) -> Self {
        Self {
            insecure: value,
            ..self
        }
    }

    pub fn build(self) -> Connector {
        let mut connector = native_tls::TlsConnector::builder();

        if self.insecure {
            connector.danger_accept_invalid_certs(true);
        } else {
            // Work out cert
            unimplemented!();
        }

        let connector = connector.build().unwrap();
        let tls = tokio_native_tls::TlsConnector::from(connector);

        Connector { tls }
    }
}
