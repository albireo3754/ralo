pub mod command;
pub mod common;

use command::{hello, history::history, world::world};
use common::{RlError, State};
use poise::Framework;
use ralo_core::log::init_log_settings;
use reqwest::{Client, Url};
use serde::Serialize;
use serenity::all::{
    ButtonStyle, ClientBuilder, CreateActionRow, CreateButton, ExecuteWebhook, GatewayIntents,
    Http, Webhook,
};
use tokio::time::Instant;
use tracing::info;

pub(crate) struct Env {
    pub token: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv::dotenv().unwrap();
        let token = std::env::var("DISCORD_API_KEY").expect("Expected a token in the environment");
        Env { token }
    }
}

async fn generate_webhook(channel: u64) -> String {
    let url = format!("https://discord.com/api/webhooks/{}", channel);

    let result = reqwest::Request::new(reqwest::Method::POST, Url::parse(&url).unwrap());

    let client = Client::new();
    let response = client.execute(result).await.unwrap();
    let result = response.text().await.unwrap();

    result
}

async fn generate_webhook_token(channel: u64, token: &str) -> String {
    let url = format!("https://discord.com/api/webhooks/{}/{}", channel, token);

    let result = reqwest::Request::new(reqwest::Method::GET, Url::parse(&url).unwrap());

    let client = Client::new();
    let response = client.execute(result).await.unwrap();
    let result = response.text().await.unwrap();

    result
}

async fn upgrade_webhook(channel: u64, id: u64, token: &str, application_id: u64) -> String {
    let url = format!("https://discord.com/api/webhooks/{}/{}", channel, token);

    let client = Client::new();
    let response = client
        .patch(url)
        .json(&RlWebhook {
            id,
            name: "Ralo wh".to_owned(),
            avatar: "".to_owned(),
            channel_id: Some(channel),
            guild_id: None,
            r#type: 3,
            application_id,
        })
        .send()
        .await
        .unwrap();
    info!("{:?}", response);
    let result = response.text().await.unwrap();

    result
}

// {
//     "type": 3,
//     "id": "658822586720976555",
//     "name": "Clyde",
//     "avatar": "689161dc90ac261d00f1608694ac6bfd",
//     "channel_id": null,
//     "guild_id": null,
//     "application_id": "658822586720976555"
//   }
#[derive(Debug, Serialize)]
struct RlWebhook {
    id: u64,
    name: String,
    avatar: String,
    channel_id: Option<u64>,
    guild_id: Option<u64>,
    r#type: u64,
    application_id: u64,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    init_log_settings();
    let env = Env::new();

    let result = generate_webhook(1261569719870423090).await;
    // let result = generate_webhook_token(
    //     1261569719870423090,
    //     "MCgBpDWGqWgoDNF-cS05_OjPid2rmQGSqethHdbfoihywd7tcu1e5sSQiMqNLYieQzNd",
    // )
    // .await;
    // let result = upgrade_webhook(
    //     1261569719870423090,
    //     1261565050900971632,
    //     "MCgBpDWGqWgoDNF-cS05_OjPid2rmQGSqethHdbfoihywd7tcu1e5sSQiMqNLYieQzNd",
    //     1261561712058826857,
    // )
    // .await;
    println!("result: {}", result);
    return;
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, "https://discord.com/api/webhooks/1261569719870423090/MCgBpDWGqWgoDNF-cS05_OjPid2rmQGSqethHdbfoihywd7tcu1e5sSQiMqNLYieQzNd")
        .await
        .expect("Replace the webhook with your own");
    let red_win = CreateButton::new(format!("{}", 1))
        .label("레드팀 승")
        .style(ButtonStyle::Danger);

    let win_row = CreateActionRow::Buttons(vec![red_win]);
    let builder = ExecuteWebhook::new()
        // .content(format!("{:?} hello there", Instant::now()))
        .components(vec![win_row])
        // .button(red_win)
        .username("Ralo Webhook");
    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");

    // let db_manager: SupabaseDBManager = SupabaseDBManager::new().await;

    let token = env.token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // let db_ref = Arc::new(db_manager);
    // let db_ref2 = db_ref.clone();
    let framework: Framework<State, RlError> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), world(), history()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State {
                    // id: 3,
                    // player_manager: player_manager::PlayerManager::new(db_ref),
                })
            })
        })
        .build();

    // tokio::spawn(async move {
    //     loop {
    //         sleep(tokio::time::Duration::from_millis(1000)).await;
    //         println!("1000 ms have elapsed");
    //     }
    // });

    // let db_ref: Arc<SupabaseDBManager> = db_ref2;
    // tokio::spawn(async move {
    //     let player_manager: player_manager::PlayerManager<SupabaseDBManager> =
    //         player_manager::PlayerManager::<SupabaseDBManager>::new(db_ref);
    //     loop {
    //         let _ = player_manager.find_all_player().await;
    //         sleep(tokio::time::Duration::from_secs(60 * 60 * 24)).await;
    //     }
    // });

    tokio::spawn(async move {
        let app = axum::Router::new()
            .route("/", axum::routing::get(|| async { "Hello, world!" }))
            .route(
                "/health",
                axum::routing::get(|| async { axum::http::StatusCode::NO_CONTENT }),
            );
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    println!("server will start");
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to create client: {:?}", e);
        });

    let result = client.start().await;

    println!("${:?}", result);
}
