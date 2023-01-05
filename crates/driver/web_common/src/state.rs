use std::sync::Arc;

use adapter_repositories_postgres::*;
use adapter_services::{
    config::TomlConfigService, crypto::hash::Argon2CryptoHashService,
    entropy::SecureEntropyService,
    link::message_passing::RabbitMqMessagePassingService,
};
use app_services::{
    auth::AppAuthService, comm::chats::AppChatsService,
    link::channels::AppChannelsService, setup::AppSetupService,
};
use kernel_repositories::DataStore;
use kernel_services::{
    auth::AuthService,
    comm::chats::ChatsService,
    config::ConfigService,
    crypto::hash::CryptoHashService,
    entropy::EntropyService,
    link::{channels::ChannelsService, message_passing::MessagePassingService},
    setup::SetupService,
    Service,
};

pub type AppState = Arc<
    AppStateImpl<
        TomlConfigService,
        SecureEntropyService,
        Argon2CryptoHashService<'static>,
        RabbitMqMessagePassingService,
        AppAuthService<TomlConfigService>,
        AppSetupService,
        AppChannelsService<RabbitMqMessagePassingService>,
        AppChatsService<RabbitMqMessagePassingService>,
    >,
>;

pub struct AppStateImpl<
    Config: ConfigService,
    Entropy: EntropyService,
    CryptoHash: CryptoHashService,
    MessagePassing: MessagePassingService,
    Auth: AuthService,
    Setup: SetupService,
    Channels: ChannelsService,
    Chats: ChatsService,
> {
    pub data: Arc<dyn DataStore>,
    pub config: Arc<Config>,
    pub entropy: Arc<Entropy>,
    pub hash: Arc<CryptoHash>,
    pub ipc: Arc<MessagePassing>,
    pub auth: Arc<Auth>,
    pub setup: Arc<Setup>,
    pub channels: Arc<Channels>,
    pub chats: Arc<Chats>,
}

pub async fn get_config_service() -> anyhow::Result<Arc<TomlConfigService>> {
    init(TomlConfigService::default()).await
}

pub async fn create_state<'a>(
    config: Arc<TomlConfigService>,
) -> anyhow::Result<AppState> {
    debug!("creating datastore");
    let conf = config.get_section::<DataConfig>(DATA_CONFIG_SECTION)?;
    let data = create_datastore(conf).await?;

    debug!("creating base services");
    let entropy = init(SecureEntropyService::default()).await?;
    let hash = init(Argon2CryptoHashService::default()).await?;
    let ipc =
        init(RabbitMqMessagePassingService::create(config.clone()).await?)
            .await?;

    debug!("creating app services");
    let auth = Arc::new(AppAuthService::new(
        data.clone(),
        config.clone(),
        hash.clone(),
        entropy.clone(),
    ));
    let setup = init(AppSetupService::new(data.clone(), auth.clone())).await?;
    let channels =
        init(AppChannelsService::new(data.clone(), ipc.clone())).await?;
    let chats =
        init(AppChatsService::create(ipc.clone(), channels.clone()).await?)
            .await?;

    debug!("building application state");
    Ok(Arc::new(AppStateImpl {
        data,
        config,
        entropy,
        hash,
        ipc,
        auth,
        setup,
        channels,
        chats,
    }))
}

async fn init<S: Service + Send + Sync>(svc: S) -> anyhow::Result<Arc<S>> {
    svc.initialize().await?;

    Ok(Arc::new(svc))
}
