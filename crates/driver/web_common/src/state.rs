use std::sync::Arc;

use adapter_repositories_postgres::*;
use adapter_services::config::TomlConfigService;
use adapter_services::crypto::hash::Argon2CryptoHashService;
use adapter_services::entropy::SecureEntropyService;
use app_services::auth::config::{AuthConfig, AUTH_CONFIG_SECTION};
use app_services::auth::AppAuthService;
use app_services::setup::AppSetupService;
use kernel_repositories::DataStore;
use kernel_services::auth::AuthService;
use kernel_services::config::ConfigService;
use kernel_services::crypto::hash::CryptoHashService;
use kernel_services::entropy::EntropyService;
use kernel_services::get_config;
use kernel_services::setup::SetupService;

pub type AppState = Arc<AppStateImpl>;

pub struct AppStateImpl {
    pub data: Arc<dyn DataStore>,

    // base services
    pub config: Arc<dyn ConfigService>,
    pub entropy: Arc<dyn EntropyService>,
    pub hash: Arc<dyn CryptoHashService>,

    // services
    pub auth: Arc<dyn AuthService>,
    pub setup: Arc<dyn SetupService>,
}

pub async fn create_state() -> anyhow::Result<AppState> {
    debug!(
        "loading config from toml files: {:?}",
        TomlConfigService::get_config_files()?
    );

    let config = TomlConfigService::load()?;

    debug!("creating datastore");
    let conf = get_config!(config, DATA_CONFIG_SECTION => DataConfig)?;
    let data = create_sqlx_datastore(conf).await?;

    debug!("creating base services");
    let config = Arc::new(config);
    let entropy = Arc::new(SecureEntropyService::default());
    let hash = Arc::new(Argon2CryptoHashService::new());

    debug!("creating app services");
    // auth
    let conf = get_config!(config, AUTH_CONFIG_SECTION => AuthConfig)?;
    let auth = Arc::new(AppAuthService::new(
        conf,
        data.clone(),
        hash.clone(),
        entropy.clone(),
    ));
    // setup
    let setup = Arc::new(AppSetupService::new(data.clone(), auth.clone()));

    debug!("building application state");
    Ok(Arc::new(AppStateImpl {
        data,
        config,
        entropy,
        hash,
        auth,
        setup,
    }))
}
