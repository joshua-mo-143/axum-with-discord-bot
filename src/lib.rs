use axum::Router;
use poise::serenity_prelude as serenity;
use poise::FrameworkBuilder;
use shuttle_secrets::SecretStore;

mod commands;
use commands::age;

mod router;
use router::build_router;
use sync_wrapper::SyncWrapper;

pub struct Data {}
pub struct CustomService {
    discord_bot:
        FrameworkBuilder<Data,Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
    // discord_bot:
    //     FrameworkBuilder<Data, shuttle_service::error::CustomError>,
    router: SyncWrapper<Router>,
}

#[shuttle_service::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> Result<CustomService, shuttle_service::Error> {
    let discord_api_key = secrets.get("DISCORD_API_KEY").unwrap();

    let discord_bot = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .token(discord_api_key)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    let router = build_router();
    let router = SyncWrapper::new(router);

    Ok(CustomService {
        discord_bot,
        router
    })
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for CustomService {
    async fn bind(
        mut self: Box<Self>,
        addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        let router = self.router.into_inner();

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select!(
            _ = self.discord_bot.run() => {},
            _ = serve_router => {}
        );

        Ok(())
    }
}