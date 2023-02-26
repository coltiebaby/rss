pub struct Connector {
    socket: tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>,
}

impl Connector {
    pub fn spawn(self) -> (flume::Receiver<crate::core::Incoming>, tokio::task::JoinHandle<()>) {
        let mut socket = self.socket;

        let (tx, rx) = flume::unbounded();

        let handle = tokio::task::spawn(async move {
            socket.write_message(tungstenite::Message::Binary("[5, \"OnJsonApiEvent\"]".into())).unwrap();
            loop {
                let msg = socket.read_message().expect("Error reading message");
                let msg = msg.to_string();
                let msg = msg.trim();

                if msg.is_empty() {
                    continue;
                }

                let incoming: crate::core::Incoming = serde_json::from_str(msg).expect("should have a new message");
                tx.send(incoming).unwrap();
            }
        });

        (rx, handle)
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
    pub fn connect(self, req: tungstenite::http::Request<()>) -> Connector {
        let uri = req.uri();

        let host = uri.host().unwrap();
        let port = uri.port().unwrap();
        let combo = format!("{host}:{port}");

        let mut connector = native_tls::TlsConnector::builder();

        if self.insecure {
            connector.danger_accept_invalid_certs(true);
        } else {
            // Work out cert
            unimplemented!();
        }

        let connector = connector.build().unwrap();

        // Do the connection
        let stream = std::net::TcpStream::connect(&combo).unwrap();
        let stream = connector.connect(&combo, stream).unwrap();
        let (socket, _) = tungstenite::client(req, stream).expect("Can't connect");

        Connector { socket }
    }

    pub fn insecure(self, value: bool) -> Self {
        Self {
            insecure: value,
            ..self
        }
    }
}
