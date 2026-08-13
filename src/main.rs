use clap::{Args, Parser, Subcommand};
use reqwest::{Client, Method, StatusCode};
use serde_json::{json, Value};
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
            client: Client::new(),
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
    ) -> Result<Value, CliError> {
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
                message: if text.is_empty() {
                    status.to_string()
                } else {
                    text
                },
            });
        }
        if text.is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_str(&text).map_err(|e| CliError::Api {
            status,
            message: format!("invalid JSON response: {e}"),
        })
    }
}

fn path_server(server: &str, suffix: &str) -> String {
    format!("/api/client/servers/{server}{suffix}")
}
async fn execute(api: &Api, command: Command) -> Result<Value, CliError> {
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
async fn search(
    api: &Api,
    server_id: &str,
    provider: &str,
    page: u32,
    page_size: u32,
    search_query: Option<&str>,
    minecraft_version: Option<&str>,
    kind: &str,
) -> Result<Value, CliError> {
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.token.filter(|t| !t.trim().is_empty()) {
        Some(token) => execute(&Api::new(cli.base_url, token), cli.command).await,
        None => Err(CliError::MissingToken),
    };
    match result {
        Ok(value) => println!(
            "{}",
            if cli.json {
                value.to_string()
            } else {
                serde_json::to_string_pretty(&value).unwrap()
            }
        ),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_paths_are_exact() {
        assert_eq!(
            path_server("srv-1", "/files/search"),
            "/api/client/servers/srv-1/files/search"
        );
    }

    #[test]
    fn empty_success_body_is_null() {
        let value: Value = serde_json::from_str("null").unwrap();
        assert!(value.is_null());
    }
}
