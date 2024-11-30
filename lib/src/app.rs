use crate::auth::AuthService;

pub trait App {
    type Auth: AuthService;

    fn auth(&self) -> &Self::Auth;
}

pub struct NewApp<Auth>
where
    Auth: AuthService,
{
    pub auth: Auth,
}

impl<Auth> App for NewApp<Auth>
where
    Auth: AuthService,
{
    type Auth = Auth;

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }
}
