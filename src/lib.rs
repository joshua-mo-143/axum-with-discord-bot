use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use poise::FrameworkBuilder;
use axum::Router;

mod commands;
use commands::age;

mod router;
use router::{router};
use sync_wrapper::SyncWrapper;

pub struct Data {}
pub struct CustomService {
    discord_bot: FrameworkBuilder<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
    router: SyncWrapper<Router>
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

    let router = router();
    let syncwrapper = SyncWrapper::new(router);

    Ok(CustomService { discord_bot, router: syncwrapper })
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for CustomService {
    async fn bind(
        mut self: Box<Self>,
        addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_service::error::Error> {
        self.start(addr).await.expect("Something went wrong!");

        Ok(())
    }
}

impl CustomService {

    async fn start(self, addr: std::net::SocketAddr) -> Result<SyncWrapper<Router>, Box<dyn std::error::Error>> {
        tokio::spawn(async move {
            self.discord_bot.run().await.unwrap();
        });

        Ok(self.router)
    }
}
