use std::process;

use base64::prelude::*;
use tungstenite::http;
use tungstenite::client::IntoClientRequest;

use super::Error;

#[derive(Debug)]
pub struct Client {
    pub token: String,
    port: String,
}

impl Client {
    pub fn new() -> super::LCUResult<Self> {
        let processes = from_process("LeagueClientUx").ok_or(super::Error::AppNotRunning)?;
        let process = processes.get(0).ok_or(super::Error::AppNotRunning)?;

        Self::from_str(process)
    }

    fn from_str(value: &str) -> super::LCUResult<Client> {
        let re = regex::Regex::new(r"--remoting-auth-token=([\w-]*) --app-port=([0-9]*)").unwrap();
        let caps = re.captures(value);
        let caps = caps.unwrap();
        let token: String = caps.get(1).unwrap().as_str().to_string();
        let port: String = caps.get(2).unwrap().as_str().to_string();

        Ok(Client { token, port })
    }

    pub fn auth(&self) -> String {
        let auth = format!("riot:{}", self.token);
        format!("basic {}", BASE64_STANDARD.encode(auth))
    }

    pub fn uri(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    pub fn req(&self) -> super::LCUResult<http::Request<()>> {
        let auth = self.auth();
        let mut req = format!("wss://127.0.0.1:{}", self.port)
            .into_client_request().unwrap();

        req.headers_mut().insert("authorization", auth.parse().unwrap());

        Ok(req)

        // http::Request::builder()
        //     .uri(format!("wss://127.0.0.1:{}", self.port))
        //     .method(http::Method::GET)
        //     .header("authorization", &auth)
        //     .header("sec-websocket-key", key)
        //     .header("Connection", "Upgrade")
        //     .header("Upgrade", "websocket")
        //     .header("Host", "server.example.com")
        //     .header("Origin", "http://echo.websocket.org")
        //     .header("sec-websocket-version", "13")
        //     .body(())
        //     .map_err(Error::Request)
    }
}

fn from_process(process: &str) -> Option<Vec<String>> {
    let ps = process::Command::new("ps")
        .args(["x", "-A", "-o args"])
        .stdout(process::Stdio::piped())
        .spawn()
        .ok()?;

    let mut grep = process::Command::new("grep");
    grep.arg(process).stdin(ps.stdout?);

    let output = String::from_utf8(grep.output().ok()?.stdout).ok()?;
    let lines = output.lines();

    let lines: Vec<String> = lines
        .filter(|x| x.contains("--app-port") && x.contains("--remoting-auth-token"))
        .map(String::from)
        .collect();

    Some(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_from_string() {
        let example = "/Users/Shared/Riot Games/Riot Client.app/Contents/Frameworks/RiotClient.app/Contents/MacOS/RiotClientUx --app-port=12345 --remoting-auth-token=token --app-pid=app-id --log-dir=/Users/crobertson/Library/Logs/Riot Games/Riot Client --user-data-root=/Users/crobertson/Library/Application Support/Riot Games/Riot Client --app-root=/Users/Shared/Riot Games/Riot Client.app --crashpad-environment=KeystoneFoundationLiveMac";

        let client = Client::from_str(example).expect("usable client");
        assert_eq!(client.port, "12345".to_string())
    }
}
