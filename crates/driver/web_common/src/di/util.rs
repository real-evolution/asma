pub mod axum {
    use shaku_axum::Inject;
    use crate::di::DI;

    pub type Dep<T> = Inject<dyn DI, T>;
}
