use std::process::{Command, Stdio};
use http_auth_basic::Credentials;
use reqwest::{Url, header};

#[derive(Debug, Default)]
pub struct RiotClient {
    pub token: String,
    pub port: String,
    pub client: reqwest::Client,
}

impl RiotClient {
    pub fn new() -> RiotClient {
        let mut rc = RiotClient { ..Default::default() };

        let wanted: Vec<String> = vec![
            "--app-port".to_string(),
            "--remoting-auth-token".to_string(),
        ];

        let process = match from_process("RiotClientUx".to_string(), wanted) {
            Some(x) => x,
            None => {
                return rc;
            }
        };

        if process.len() != 2 {
            return rc;
        }

        rc.port = process[0].clone();
        rc.token = process[1].clone();

        let credentials = Credentials::new("riot", &rc.token);
        let credentials = credentials.as_http_header();

        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&credentials).unwrap());
        rc.client = reqwest::Client::builder().default_headers(headers).danger_accept_invalid_certs(true).build().unwrap();

        rc
    }

    pub fn get_url_builder(&self) -> Url {
        Url::parse(&format!("https://127.0.0.1:{port}", port=self.port)).unwrap()
    }

    pub fn swagger_url(&self) -> Url {
        Url::parse(&format!("https://@127.0.0.1:{port}/swagger/v3/openapi.json", port=self.port)).unwrap()
    }

    // let client = Client::new(installed_at).expect("failed to look at lockfile");
    // let (tx, rx) = mpsc::channel::<String>();

    // std::thread::spawn(move || client.watcher(tx));
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    // fn watcher(self, tx: mpsc::Sender<String>) {
    //     let dur = std::time::Duration::from_secs(1);

    //     let installed = path::Path::new(&self.installed_at);
    //     let lockfile = installed.join("lockfile");

    //     while lockfile.is_file() {
    //         std::thread::sleep(dur);
    //     }

    //     tx.send("no file".to_string());
    // }
}

fn from_process(process: String, need: Vec<String>) -> Option<Vec<String>> {
    // Runs the command below. -o command only shows that colum.
    let ps = Command::new("ps").arg("x").arg("-A").arg("-o args").stdout(Stdio::piped()).spawn().unwrap();
    let mut grep = Command::new("grep");
    grep.arg(process);
    grep.stdin(ps.stdout.unwrap());

    let output = String::from_utf8(grep.output().unwrap().stdout).unwrap();
    let lines = output.lines();

    let mut results: Vec<String> = Vec::new();
    'liner: for line in lines {
        // Skip the line
        for n in &need {
            if !line.contains(n.as_str()) {
                continue 'liner;
            }
        }

        println!("{:?}", line);

        let spaced = line.split(" --").into_iter();

        for space in spaced {
            for n in need.iter() {
                let mut lf = n.clone();
                lf.push('=');

                let lf_str = lf.as_str().strip_prefix("--")?;
                if space.contains(lf_str) {
                    let result = space.strip_prefix(lf_str)?;
                    results.push(result.to_string());
                }

            }
        }

        break;
    }

    if results.is_empty() {
        return None
    }

    Some(results)
}

// GET /swagger/v1/api-docs
// GET /swagger/v1/api-docs/{api}
// GET /swagger/v2/swagger.json
// GET /swagger/v3/openapi.json

// struct RequestBuilder {
//     plugin: String,
//     version: String,
// }
