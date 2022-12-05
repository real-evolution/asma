mod accounts;
mod roles;
mod sessions;
mod users;

pub use accounts::*;
pub use roles::*;
pub use sessions::*;
pub use users::*;

pub trait AuthDataStore: Send + Sync {
    fn users(&self) -> &dyn UsersRepo;
    fn roles(&self) -> &dyn RolesRepo;
    fn accounts(&self) -> &dyn AccountsRepo;
    fn sessions(&self) -> &dyn SessionsRepo;
}
