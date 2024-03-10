use crate::{Error, LCUResult};

pub struct Connector {
    socket: tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>,
}

impl Connector {
    pub fn spawn(self) -> LCUResult<(flume::Receiver<crate::core::Incoming>, tokio::task::JoinHandle<()>)> {
        let mut socket = self.socket;

        let (tx, rx) = flume::unbounded();

        let handle = tokio::task::spawn(async move {
            socket.write(tungstenite::Message::Binary("[5, \"OnJsonApiEvent\"]".into())).unwrap();
            loop {
                let msg = socket.read().expect("Error reading message");
                let msg = msg.to_string();
                let msg = msg.trim();

                if msg.is_empty() {
                    continue;
                }

                let incoming: crate::core::Incoming = serde_json::from_str(msg).expect("should have a new message");
                tx.send(incoming).expect("failed to send message");
            }
        });

        Ok((rx, handle))
    }

    pub fn builder() -> ConnectorBuilder {
        ConnectorBuilder::default()
    }
}

#[derive(Default)]
pub struct ConnectorBuilder {
    insecure: bool,
}

impl ConnectorBuilder {
    pub fn connect(self, req: tungstenite::http::Request<()>) -> LCUResult<Connector> {
        let uri = req.uri();

        let host = uri.host().ok_or(Error::Uri)?;
        let port = uri.port().ok_or(Error::Uri)?;
        let combo = format!("{host}:{port}");

        let mut connector = native_tls::TlsConnector::builder();

        if self.insecure {
            connector.danger_accept_invalid_certs(true);
        } else {
            // Work out cert
            unimplemented!();
        }

        let connector = connector.build().map_err(|e| Error::Websocket(e.to_string()))?;

        // Do the connection
        let stream = std::net::TcpStream::connect(&combo).map_err(|e| Error::Websocket(e.to_string()))?;
        let stream = connector.connect(&combo, stream).map_err(|e| Error::Websocket(e.to_string()))?;
        let (socket, _) = tungstenite::client(req, stream).map_err(|e| Error::Websocket(e.to_string()))?;

        Ok(Connector { socket })
    }

    pub fn insecure(self, value: bool) -> Self {
        Self {
            insecure: value,
            ..self
        }
    }
}
