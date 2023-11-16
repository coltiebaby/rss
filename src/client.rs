use http_auth_basic::Credentials;
use reqwest::header;
use std::process;

#[derive(Debug)]
pub struct Client {
    pub(crate) token: String,
    pub(crate) port: String,
}

impl Client {
    pub fn new() -> super::LCUResult<Self> {
        let processes = from_process("RiotClientUx").ok_or(super::LCUError::AppNotRunning)?;
        let process = processes.get(0).ok_or(super::LCUError::AppNotRunning)?;

        Self::from_str(process)
    }

    fn from_str(value: &str) -> super::LCUResult<Client> {
        let re = regex::Regex::new(r"--app-port=([0-9]*).*--remoting-auth-token=([\w-]*)").unwrap();
        let caps = re.captures(value).unwrap();
        let port: String = caps.get(1).unwrap().as_str().to_string();
        let token: String = caps.get(2).unwrap().as_str().to_string();

        Ok(Client { token, port })
    }

    pub fn to_reqwest(&self) -> super::LCUResult<reqwest::Client> {
        let auth = Credentials::new("riot", &self.token);
        let auth = auth.as_http_header();
        let auth = header::HeaderValue::from_str(&auth)
            .map_err(|e| super::LCUError::HttpClientError(e.to_string()))?;

        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", auth);

        reqwest::Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| super::LCUError::HttpClientError(e.to_string()))
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
