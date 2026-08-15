use clap::{Args, Parser, Subcommand};
use reqwest::{Client, Method, StatusCode};
use serde_json::{json, Value};
use std::time::Duration;
use thiserror::Error;

const DEFAULT_BASE_URL: &str = "https://panel.pebblehost.com";

#[derive(Debug, Error)]
enum CliError {
    #[error("missing API token: set PEBBLEHOST_API_TOKEN or pass --token")]
    MissingToken,
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("PebbleHost API error ({status}): {message}")]
    Api { status: StatusCode, message: String },
}

#[derive(Parser, Debug)]
#[command(
    name = "pebblehost",
    about = "Manage PebbleHost servers from the command line"
)]
struct Cli {
    #[arg(long, env = "PEBBLEHOST_API_TOKEN", hide_env_values = true)]
    token: Option<String>,
    #[arg(long, env = "PEBBLEHOST_BASE_URL", default_value = DEFAULT_BASE_URL)]
    base_url: String,
    #[arg(long, global = true, help = "Print compact JSON output")]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[allow(clippy::enum_variant_names)]
enum Command {
    Account,
    Servers,
    Server(ServerId),
    Power(PowerArgs),
    Command(CommandArgs),
    Resources(ServerId),
    Activity(ServerId),
    Backups(ServerId),
    Databases(ServerId),
    Allocations(ServerId),
    Schedules(ServerId),
    Players(ServerId),
    Plugins(PluginArgs),
    Modpacks(ModpackArgs),
    Files(FilesArgs),
    FileSearch(FileSearchArgs),
    File(FileArgs),
}

#[derive(Args, Debug)]
struct ServerId {
    server_id: String,
}
#[derive(Args, Debug)]
struct PowerArgs {
    server_id: String,
    #[arg(long, value_parser = ["start", "stop", "restart", "kill"])]
    action: String,
}
#[derive(Args, Debug)]
struct CommandArgs {
    server_id: String,
    #[arg(long)]
    command: String,
}
#[derive(Args, Debug)]
struct PluginArgs {
    server_id: String,
    #[arg(long)]
    provider: String,
    #[arg(long, default_value_t = 1)]
    page: u32,
    #[arg(long, default_value_t = 20)]
    page_size: u32,
    #[arg(long)]
    search_query: Option<String>,
    #[arg(long)]
    minecraft_version: Option<String>,
}
#[derive(Args, Debug)]
struct ModpackArgs {
    server_id: String,
    #[arg(long)]
    provider: String,
    #[arg(long, default_value_t = 1)]
    page: u32,
    #[arg(long, default_value_t = 20)]
    page_size: u32,
    #[arg(long)]
    search_query: Option<String>,
}
#[derive(Args, Debug)]
struct FilesArgs {
    server_id: String,
    #[arg(long, default_value = "/")]
    directory: String,
}
#[derive(Args, Debug)]
struct FileSearchArgs {
    server_id: String,
    query: String,
    #[arg(long, default_value = "/")]
    root: String,
}
#[derive(Args, Debug)]
struct FileArgs {
    server_id: String,
    path: String,
}

struct Api {
    client: Client,
    base_url: String,
    token: String,
}

impl Api {
    fn new(base_url: String, token: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("reqwest client should build"),
            base_url: base_url.trim_end_matches('/').to_owned(),
            token,
        }
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<Value>,
    ) -> Result<Response, CliError> {
        let mut req = self
            .client
            .request(method, format!("{}{}", self.base_url, path))
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .query(query);
        if let Some(body) = body {
            req = req.json(&body);
        }
        let response = req.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(CliError::Api {
                status,
                message: if text.is_empty() { status.to_string() } else { text },
            });
        }
        if text.is_empty() {
            return Ok(Response::Json(Value::Null));
        }
        Ok(serde_json::from_str(&text)
            .map(Response::Json)
            .unwrap_or_else(|_| Response::Text(text)))
    }
}

#[derive(Debug, PartialEq)]
enum Response {
    Json(Value),
    Text(String),
}

fn path_server(server: &str, suffix: &str) -> String {
    format!("/api/client/servers/{server}{suffix}")
}

async fn execute(api: &Api, command: Command) -> Result<Response, CliError> {
    match command {
        Command::Account => {
            api.request(Method::GET, "/api/client/account", &[], None)
                .await
        }
        Command::Servers => api.request(Method::GET, "/api/client", &[], None).await,
        Command::Server(a) => {
            api.request(Method::GET, &path_server(&a.server_id, ""), &[], None)
                .await
        }
        Command::Power(a) => {
            api.request(
                Method::POST,
                &path_server(&a.server_id, "/power"),
                &[],
                Some(json!({"signal": a.action})),
            )
            .await
        }
        Command::Command(a) => {
            api.request(
                Method::POST,
                &path_server(&a.server_id, "/command"),
                &[],
                Some(json!({"command": a.command})),
            )
            .await
        }
        Command::Resources(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/resources"),
                &[],
                None,
            )
            .await
        }
        Command::Activity(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/activity"),
                &[],
                None,
            )
            .await
        }
        Command::Backups(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/backups"),
                &[],
                None,
            )
            .await
        }
        Command::Databases(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/databases"),
                &[],
                None,
            )
            .await
        }
        Command::Allocations(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/network/allocations"),
                &[],
                None,
            )
            .await
        }
        Command::Schedules(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/schedules"),
                &[],
                None,
            )
            .await
        }
        Command::Players(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/minecraft/players"),
                &[],
                None,
            )
            .await
        }
        Command::Plugins(a) => {
            search(
                api,
                &a.server_id,
                &a.provider,
                a.page,
                a.page_size,
                a.search_query.as_deref(),
                a.minecraft_version.as_deref(),
                "plugins",
            )
            .await
        }
        Command::Modpacks(a) => {
            search(
                api,
                &a.server_id,
                &a.provider,
                a.page,
                a.page_size,
                a.search_query.as_deref(),
                None,
                "modpacks",
            )
            .await
        }
        Command::Files(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/files/list"),
                &[("directory", a.directory)],
                None,
            )
            .await
        }
        Command::FileSearch(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/files/search"),
                &[("root", a.root), ("query", a.query)],
                None,
            )
            .await
        }
        Command::File(a) => {
            api.request(
                Method::GET,
                &path_server(&a.server_id, "/files/contents"),
                &[("file", a.path)],
                None,
            )
            .await
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn search(
    api: &Api,
    server_id: &str,
    provider: &str,
    page: u32,
    page_size: u32,
    search_query: Option<&str>,
    minecraft_version: Option<&str>,
    kind: &str,
) -> Result<Response, CliError> {
    let mut q = vec![
        ("provider", provider.to_owned()),
        ("page", page.to_string()),
        ("page_size", page_size.to_string()),
    ];
    if let Some(v) = search_query {
        q.push(("search_query", v.to_owned()));
    }
    if let Some(v) = minecraft_version {
        q.push(("minecraft_version", v.to_owned()));
    }
    api.request(
        Method::GET,
        &path_server(server_id, &format!("/minecraft/{kind}")),
        &q,
        None,
    )
    .await
}

async fn run(cli: Cli) -> Result<Response, CliError> {
    let token = cli
        .token
        .as_ref()
        .filter(|t| !t.trim().is_empty())
        .ok_or(CliError::MissingToken)?;
    execute(&Api::new(cli.base_url, token.to_owned()), cli.command).await
}

fn sort_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(k, v)| (k, sort_value(v)))
                    .collect(),
            )
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_value).collect()),
        other => other,
    }
}

fn print_response(response: Response, json: bool) {
    match response {
        Response::Json(value) => {
            let sorted = sort_value(value);
            let output = if json {
                sorted.to_string()
            } else {
                serde_json::to_string_pretty(&sorted).unwrap()
            };
            println!("{}", output);
        }
        Response::Text(text) => println!("{}", text),
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let as_json = cli.json;
    match run(cli).await {
        Ok(response) => print_response(response, as_json),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::{body_json, header, method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    fn test_api(server: &MockServer, token: &str) -> Api {
        Api::new(server.uri(), token.to_owned())
    }

    fn cli_with(token: &str, base_url: String, command: Command) -> Cli {
        Cli {
            token: Some(token.to_owned()),
            base_url,
            json: false,
            command,
        }
    }

    #[tokio::test]
    async fn api_uses_bearer_and_decodes_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client"))
            .and(header("Authorization", "Bearer secret"))
            .and(header("Accept", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": []})))
            .mount(&server)
            .await;
        let api = test_api(&server, "secret");
        let resp = api.request(Method::GET, "/api/client", &[], None).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"data": []})));
    }

    #[tokio::test]
    async fn api_reports_api_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client/account"))
            .and(header("Authorization", "Bearer secret"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(json!({"errors": [{"detail": "Unauthenticated"}]})),
            )
            .mount(&server)
            .await;
        let api = test_api(&server, "secret");
        let err = api
            .request(Method::GET, "/api/client/account", &[], None)
            .await
            .unwrap_err();
        match err {
            CliError::Api { status, message } => {
                assert_eq!(status, 401);
                assert!(message.contains("Unauthenticated"));
            }
            _ => panic!("expected Api error, got {err:?}"),
        }
    }

    #[tokio::test]
    async fn command_sends_user_command_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/client/servers/srv-1/command"))
            .and(header("Authorization", "Bearer secret"))
            .and(body_json(json!({"command": "say hello"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::Command(CommandArgs {
                server_id: "srv-1".into(),
                command: "say hello".into(),
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"ok": true})));
    }

    #[tokio::test]
    async fn power_sends_signal_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/client/servers/srv-1/power"))
            .and(header("Authorization", "Bearer secret"))
            .and(body_json(json!({"signal": "start"})))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::Power(PowerArgs {
                server_id: "srv-1".into(),
                action: "start".into(),
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(Value::Null));
    }

    #[tokio::test]
    async fn resources_path_is_exact() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client/servers/srv-1/resources"))
            .and(header("Authorization", "Bearer secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"resources": {}})))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::Resources(ServerId {
                server_id: "srv-1".into(),
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"resources": {}})));
    }

    #[tokio::test]
    async fn plugins_sends_documented_query() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client/servers/srv-1/minecraft/plugins"))
            .and(header("Authorization", "Bearer secret"))
            .and(query_param("provider", "modrinth"))
            .and(query_param("page", "2"))
            .and(query_param("page_size", "10"))
            .and(query_param("search_query", "worldedit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": []})))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::Plugins(PluginArgs {
                server_id: "srv-1".into(),
                provider: "modrinth".into(),
                page: 2,
                page_size: 10,
                search_query: Some("worldedit".into()),
                minecraft_version: None,
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"data": []})));
    }

    #[tokio::test]
    async fn search_files_sends_documented_parameters() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client/servers/srv-1/files/search"))
            .and(header("Authorization", "Bearer secret"))
            .and(query_param("root", "/plugins"))
            .and(query_param("query", "paper"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": []})))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::FileSearch(FileSearchArgs {
                server_id: "srv-1".into(),
                query: "paper".into(),
                root: "/plugins".into(),
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"data": []})));
    }

    #[tokio::test]
    async fn raw_text_success_body_is_preserved() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client/servers/srv-1/files/contents"))
            .and(header("Authorization", "Bearer secret"))
            .and(query_param("file", "server.properties"))
            .respond_with(ResponseTemplate::new(200).set_body_string("motd=A Minecraft Server\n"))
            .mount(&server)
            .await;
        let cli = cli_with(
            "secret",
            server.uri(),
            Command::File(FileArgs {
                server_id: "srv-1".into(),
                path: "server.properties".into(),
            }),
        );
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Text("motd=A Minecraft Server\n".into()));
    }

    #[tokio::test]
    async fn run_requires_non_empty_token() {
        let cli = Cli {
            token: None,
            base_url: "http://example.test".into(),
            json: false,
            command: Command::Servers,
        };
        let err = run(cli).await.unwrap_err();
        assert!(matches!(err, CliError::MissingToken));
    }

    #[tokio::test]
    async fn json_flag_produces_compact_sorted_output() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/client"))
            .and(header("Authorization", "Bearer secret"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"z":1,"a":2}"#))
            .mount(&server)
            .await;
        let cli = Cli {
            token: Some("secret".into()),
            base_url: server.uri(),
            json: true,
            command: Command::Servers,
        };
        let resp = run(cli).await.unwrap();
        assert_eq!(resp, Response::Json(json!({"z": 1, "a": 2})));
        if let Response::Json(value) = resp {
            let sorted = sort_value(value);
            assert_eq!(serde_json::to_string(&sorted).unwrap(), r#"{"a":2,"z":1}"#);
        }
    }
}
