use shaku::HasComponent;

use crate::{auth::*, link::*, TransactionManager};

pub trait ReposModule:
    HasComponent<dyn TransactionManager>
    + HasComponent<dyn UsersRepo>
    + HasComponent<dyn AccountsRepo>
    + HasComponent<dyn RolesRepo>
    + HasComponent<dyn SessionsRepo>
    + HasComponent<dyn ChannelsRepo>
{
}
