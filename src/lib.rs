use shuttle_secrets::SecretStore;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

mod commands;
use commands::age;

mod router;
use router::router;

pub struct Data {}
pub struct CustomService {
    discord_api_key: String
}

#[shuttle_service::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> Result<CustomService, shuttle_service::Error> {

    let discord_api_key = secrets.get("DISCORD_API_KEY").unwrap();

    Ok(CustomService {
        discord_api_key
    })
}

#[shuttle_service::async_trait]
impl shuttle_service::Service for CustomService {
    async fn bind(
        mut self: Box<Self>,
        addr: std::net::SocketAddr
    ) -> Result<(), shuttle_service::error::Error> {
        
        let service = Arc::new(self);

        let router = Arc::clone(&service);

        tokio::spawn(async move {
            Arc::clone(&service).set_up_bot().await.expect("Something went wrong with the bot! :(");
        });

        router.set_up_router(addr).await.expect("Something went wrong with the router! :(");

        Ok(())
    }
}

impl CustomService {
    async fn set_up_bot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .token(&self.discord_api_key)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

        framework.run().await.unwrap();

        Ok(())
    }

    async fn set_up_router(&self, addr: std::net::SocketAddr) -> Result<(), hyper::Error> {
        let router = router();

            let meme = axum::Server::bind(&addr).serve(router.into_make_service());

        meme.await
    }
}

