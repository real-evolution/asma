use std::sync::Arc;

use adapter_repositories_postgres::*;
use adapter_services::{
    config::TomlConfigService,
    crypto::hash::Argon2CryptoHashService,
    entropy::SecureEntropyService,
};
use app_services::{
    auth::AppAuthService,
    link::channels::AppChannelsService,
    setup::AppSetupService,
};
use kernel_repositories::DataStore;
use kernel_services::{
    auth::AuthService,
    config::ConfigService,
    crypto::hash::CryptoHashService,
    entropy::EntropyService,
    link::channels::ChannelsService,
    setup::SetupService,
    Service,
};

pub type AppState = Arc<
    AppStateImpl<
        TomlConfigService,
        SecureEntropyService,
        Argon2CryptoHashService<'static>,
        AppAuthService<TomlConfigService>,
        AppSetupService,
        AppChannelsService,
    >,
>;

pub struct AppStateImpl<
    Config: ConfigService,
    Entropy: EntropyService,
    CryptoHash: CryptoHashService,
    Auth: AuthService,
    Setup: SetupService,
    Channels: ChannelsService,
> {
    pub data: Arc<dyn DataStore>,
    pub config: Arc<Config>,
    pub entropy: Arc<Entropy>,
    pub hash: Arc<CryptoHash>,
    pub auth: Arc<Auth>,
    pub setup: Arc<Setup>,
    pub channels: Arc<Channels>,
}

pub async fn create_state<'a>() -> anyhow::Result<AppState> {
    let config = init(TomlConfigService::default()).await?;

    debug!("creating datastore");
    let conf = config.get_section::<DataConfig>(DATA_CONFIG_SECTION)?;
    let data = create_datastore(conf).await?;

    debug!("creating base services");
    let entropy = init(SecureEntropyService::default()).await?;
    let hash = init(Argon2CryptoHashService::new()).await?;

    debug!("creating app services");
    let auth = Arc::new(AppAuthService::new(
        data.clone(),
        config.clone(),
        hash.clone(),
        entropy.clone(),
    ));
    let setup = init(AppSetupService::new(data.clone(), auth.clone())).await?;
    let channels = init(AppChannelsService::new(data.clone())).await?;

    debug!("building application state");
    Ok(Arc::new(AppStateImpl {
        data,
        config,
        entropy,
        hash,
        auth,
        setup,
        channels,
    }))
}

async fn init<S: Service + Send + Sync>(svc: S) -> anyhow::Result<Arc<S>> {
    svc.initialize().await?;

    Ok(Arc::new(svc))
}
