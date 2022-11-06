use driver_web_common::di::DI;
use shaku_axum::Inject;

pub(crate) type Dep<T> = Inject<dyn DI, T>;
