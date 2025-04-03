use crate::{
    auth::{AuthRepo, AuthService},
    user::{UserRepo, UserService},
};

pub trait Service: Sync + Send + 'static {}
impl<T: Sync + Send + 'static> Service for T {}

pub trait Repo: Sync + Send + 'static {}
impl<T: Sync + Send + 'static> Repo for T {}

pub trait App: Sync + Send + 'static {
    type Auth: AuthRepo;
    type User: UserRepo;

    fn auth(&self) -> &AuthService<Self::Auth>;

    fn user(&self) -> &UserService<Self::User>;
}

pub struct NewApp<Auth, User>
where
    Auth: AuthRepo,
    User: UserRepo,
{
    pub auth: AuthService<Auth>,
    pub user: UserService<User>,
}

impl<Auth, User> App for NewApp<Auth, User>
where
    Auth: AuthRepo,
    User: UserRepo,
{
    type Auth = Auth;
    type User = User;

    fn auth(&self) -> &AuthService<Self::Auth> {
        &self.auth
    }

    fn user(&self) -> &UserService<Self::User> {
        &self.user
    }
}
