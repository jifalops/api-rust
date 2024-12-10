use schemars::JsonSchema;

use crate::{
    auth::AuthService,
    user::{UserRepoAdapter, UserService},
};

pub trait Service: Sync + Send + 'static {}
impl<T: Sync + Send + 'static> Service for T {}

pub trait App: Sync + Send + 'static {
    type Auth: AuthService;
    type UserRepo: UserRepoAdapter;

    fn auth(&self) -> &Self::Auth;

    fn user(&self) -> &UserService<Self::UserRepo>;
}

#[derive(JsonSchema)]
pub struct NewApp<Auth, UserRepo>
where
    Auth: AuthService,
    UserRepo: UserRepoAdapter,
{
    pub auth: Auth,
    pub user: UserService<UserRepo>,
}

impl<Auth, UserRepo> App for NewApp<Auth, UserRepo>
where
    Auth: AuthService,
    UserRepo: UserRepoAdapter,
{
    type Auth = Auth;
    type UserRepo = UserRepo;

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }

    fn user(&self) -> &UserService<Self::UserRepo> {
        &self.user
    }
}
