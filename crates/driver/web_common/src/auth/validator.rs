use kernel_entities::{
    entities::auth::{Account, Actions, Resource, User},
    traits::Key,
};

pub trait AuthValidator: Sized {
    type Error;

    fn require<F: FnOnce() -> bool>(
        &self,
        req: F,
    ) -> Result<&Self, Self::Error>;

    fn is_root(&self) -> Result<&Self, Self::Error>;

    fn in_role<'a, R: Into<&'a str>>(
        &self,
        role: R,
    ) -> Result<&Self, Self::Error>;

    fn can<A: Into<Actions> + Copy>(
        &self,
        perms: &[(Resource, A)],
    ) -> Result<&Self, Self::Error>;

    fn is(&self, account_id: &Key<Account>) -> Result<&Self, Self::Error>;

    fn of(&self, user_id: &Key<User>) -> Result<&Self, Self::Error>;

    fn unauthorized(&self) -> Result<&Self, Self::Error>;
}
