use derive_more::Constructor;
use kernel_entities::{entities::auth::*, traits::Key};

use crate::error::{ApiError, ApiResult};
use crate::util::claims::Claims;

#[derive(Constructor)]
pub struct ClaimsRequirement<'a>(&'a Claims);

impl<'a> ClaimsRequirement<'a> {
    #[inline]
    pub fn is_root(self) -> ApiResult<Self> {
        if !self.0.config.disable_root
            && self.0.roles.contains_key(KnownRoles::Root.into())
        {
            return Ok(self);
        }

        Self::insufficient_permissions()
    }

    #[inline]
    pub fn in_role<R: Into<&'a str>>(self, role: &R) -> ApiResult<Self> {
        self.is_root()
            .or(self.require(|| self.0.roles.contains_key((*role).into())))
    }

    #[inline]
    pub fn in_all<R: Into<&'a str>>(self, roles: &[R]) -> ApiResult<Self> {
        self.is_root()
            .or(self.require(|| roles.iter().all(|r| self.has_role(r))))
    }

    #[inline]
    pub fn in_any<R: Into<&'a str>>(self, roles: &[R]) -> ApiResult<Self> {
        self.is_root()
            .or(self.require(|| roles.iter().any(|r| self.has_role(r))))
    }

    #[inline]
    pub fn can<A: Into<Actions>>(
        self,
        resource: Resource,
        actions: A,
    ) -> ApiResult<Self> {
        self.is_root().or(self.require(|| {
            let actions: Actions = actions.into();

            self.0.roles.iter().any(|r| {
                r.1.iter()
                    .any(|a| a.0 == resource && a.1.has(actions.into()))
            })
        }))
    }

    #[inline]
    pub fn require<F: Fn() -> bool>(self, req: F) -> ApiResult<Self> {
        if req() {
            Ok(self)
        } else {
            Self::insufficient_permissions()
        }
    }

    #[inline]
    pub fn in_role_with<R: Into<&'a str>, A: Into<Actions>>(
        self,
        role: &R,
        perms: &[(Resource, A)],
    ) -> ApiResult<Self> {
        let Some(role_perms) = self.0.roles.get((*role).into()) else {
            return Self::insufficient_permissions();
        };

        self.require(|| {
            perms.iter().all(|p| {
                role_perms
                    .iter()
                    .any(|rp| p.0 == rp.0 && rp.1.has(p.1.into()))
            })
        })
    }

    #[inline]
    pub fn is(self, account_id: &Key<Account>) -> ApiResult<Self> {
        self.require(|| self.0.account_id.value_ref() == account_id.value_ref())
    }

    #[inline]
    pub fn of(self, user_id: &Key<User>) -> ApiResult<Self> {
        self.require(|| self.0.user_id.value_ref() == user_id.value_ref())
    }

    #[inline]
    fn has_role<R: Into<&'a str>>(&self, role: &R) -> bool {
        self.0.roles.contains_key((*role).into())
    }

    #[inline]
    fn insufficient_permissions() -> ApiResult<Self> {
        Err(ApiError::Authorization("insufficient permissions".into()))
    }
}
