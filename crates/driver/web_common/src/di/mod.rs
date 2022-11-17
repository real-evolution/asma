mod base_services;
mod repos;
mod root;
mod services;

pub use base_services::base_services_module;
pub use root::build_root as build_di;
pub use root::RootModule as DI;
