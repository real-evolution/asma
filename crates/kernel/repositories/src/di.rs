use crate::*;

use shaku::HasComponent;

pub trait ReposModule:
    HasComponent<dyn UsersRepo>
    + HasComponent<dyn AccountsRepo>
    + HasComponent<dyn RolesRepo>
    + HasComponent<dyn SessionsRepo>
{
}
