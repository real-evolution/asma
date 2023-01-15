use aide::OperationIo;
use derive_more::{Deref, From, Into};
use driver_web_common::auth::{token::AuthToken, validator::AuthValidator};
use kernel_entities::{
    entities::auth::{Account, Actions, KnownRoles, Resource, User},
    traits::Key,
};

use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Deref, From, Into, OperationIo)]
#[aide(input)]
#[repr(transparent)]
pub struct RestAuthToken(AuthToken);

impl AuthValidator for RestAuthToken {
    type Error = ApiError;

    #[inline]
    fn require<F: FnOnce() -> bool>(&self, req: F) -> ApiResult<&Self> {
        if req() {
            Ok(self)
        } else {
            Self::insufficient_permissions()
        }
    }

    #[inline]
    fn is_root(&self) -> ApiResult<&Self> {
        if !self.config.disable_root
            && self.roles.contains(KnownRoles::Root.into())
        {
            return Ok(self);
        }

        Self::insufficient_permissions()
    }

    fn in_role<'a, R: Into<&'a str>>(&self, role: R) -> ApiResult<&Self> {
        self.is_root()
            .or_else(|_| self.require(|| self.roles.contains(role.into())))
    }

    fn can<A: Into<Actions> + Copy>(
        &self,
        perms: &[(Resource, A)],
    ) -> ApiResult<&Self> {
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

    fn is(&self, account_id: &Key<Account>) -> ApiResult<&Self> {
        self.require(|| self.account_id.value_ref() == account_id.value_ref())
    }

    fn of(&self, user_id: &Key<User>) -> ApiResult<&Self> {
        self.require(|| self.user_id.value_ref() == user_id.value_ref())
    }
}

impl RestAuthToken {
    #[inline]
    fn insufficient_permissions() -> ApiResult<&'static Self> {
        Err(ApiError::Authorization("insufficient permissions".into()))
    }
}