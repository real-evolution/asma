use shaku::HasComponent;

use crate::*;

pub trait ReposModule:
    HasComponent<dyn TransactionManager>
    + HasComponent<dyn UsersRepo>
    + HasComponent<dyn AccountsRepo>
    + HasComponent<dyn RolesRepo>
    + HasComponent<dyn SessionsRepo>
{
}
