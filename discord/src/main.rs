pub mod command;
pub mod common;

use common::State;
use poise::Framework;
use serenity::all::{ClientBuilder, GatewayIntents};

pub(crate) struct Env {
    pub token: String,
}

impl Env {
    pub fn new() -> Self {
        dotenv::dotenv().unwrap();
        let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        Env { token }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let env = Env::new();

    // let db_manager: SupabaseDBManager = SupabaseDBManager::new().await;

    let token = env.token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // let db_ref = Arc::new(db_manager);
    // let db_ref2 = db_ref.clone();
    let framework: Framework<State, anyhow::Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // commands: vec![
            //     // enroll_controller::enroll(),
            //     // make_game_controller::make_game(),
            //     // autocomplete::ahri(),
            //     // make_game_controller::test_reuse_response(),
            //     // make_game_controller::add()
            //     // board_controller::board(),
            // ],
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
        .await;
    let result = client.expect("fail to start").start().await;

    println!("${:?}", result);
}
