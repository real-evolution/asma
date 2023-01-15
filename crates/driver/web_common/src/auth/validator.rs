use std::ops::Deref;

use kernel_entities::{
    entities::auth::{Account, Actions, KnownRoles, Resource, User},
    traits::Key,
};

use super::token::AuthToken;

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
}

pub trait FallbackValidator: Sized {
    type Error;

    fn unauthorized(&self) -> Result<&Self, Self::Error>;
}

impl<T, E> AuthValidator for T
where
    T: Deref<Target = AuthToken> + FallbackValidator<Error = E>,
{
    type Error = E;

    #[inline]
    fn require<F: FnOnce() -> bool>(
        &self,
        req: F,
    ) -> Result<&Self, Self::Error> {
        if req() {
            return Ok(self);
        }

        self.unauthorized()
    }

    #[inline]
    fn is_root(&self) -> Result<&Self, Self::Error> {
        if !self.config.disable_root
            && self.roles.contains(KnownRoles::Root.into())
        {
            return Ok(self);
        }

        self.unauthorized()
    }

    #[inline]
    fn in_role<'a, R: Into<&'a str>>(
        &self,
        role: R,
    ) -> Result<&Self, Self::Error> {
        self.is_root()
            .or_else(|_| self.require(|| self.roles.contains(role.into())))
    }

    #[inline]
    fn can<A: Into<Actions> + Copy>(
        &self,
        perms: &[(Resource, A)],
    ) -> Result<&Self, Self::Error> {
        self.is_root().or_else(|_| {
            self.require(|| {
                perms
                    .iter()
                    .all(|(res, act)| match self.permissions.get(res) {
                        | Some(p) => p.has(act),
                        | None => false,
                    })
            })
        })
    }

    #[inline]
    fn is(&self, account_id: &Key<Account>) -> Result<&Self, Self::Error> {
        self.require(|| self.account_id.value_ref() == account_id.value_ref())
    }

    #[inline]
    fn of(&self, user_id: &Key<User>) -> Result<&Self, Self::Error> {
        self.require(|| self.user_id.value_ref() == user_id.value_ref())
    }
}
