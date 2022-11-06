use std::sync::Arc;

use adapter_services::{
    config::TomlConfigService,
    crypto::hash::{Argon2CryptoHashService, CryptoHashServiceImplParameters},
    entropy::{EntropyServiceImplParameters, SecureEntropyService},
};
use kernel_services::{
    config::ConfigService, crypto::hash::CryptoHashService,
    entropy::EntropyService,
};
use shaku::{module, HasComponent};

pub trait BaseServicesModule:
    HasComponent<dyn ConfigService>
    + HasComponent<dyn EntropyService>
    + HasComponent<dyn CryptoHashService>
{
}

module! {
    pub BaseServicesModuleImpl: BaseServicesModule {
        components = [
            TomlConfigService,
            SecureEntropyService,
            Argon2CryptoHashService<'static>
        ],
        providers = [],
    }
}

pub fn base_services_module() -> anyhow::Result<Arc<dyn BaseServicesModule>> {
    let loaded_config = Box::new(TomlConfigService::load()?);

    Ok(Arc::new(
        BaseServicesModuleImpl::builder()
            .with_component_override::<dyn ConfigService>(loaded_config)
            .with_component_parameters::<SecureEntropyService>(
                EntropyServiceImplParameters {
                    rng: Default::default(),
                },
            )
            .with_component_parameters::<Argon2CryptoHashService<'static>>(
                CryptoHashServiceImplParameters {
                    hasher: Default::default(),
                },
            )
            .build(),
    ))
}
