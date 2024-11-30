use crate::auth::AuthService;

pub trait Service: Clone + Send + Sync + 'static {}
impl<T: Clone + Sync + Send + 'static> Service for T {}

pub trait Services {
    type Auth: AuthService;

    fn auth(&self) -> &Self::Auth;
}

#[derive(Clone)]
pub struct App<Auth>
where
    Auth: AuthService,
{
    auth: Auth,
}

impl<Auth> App<Auth>
where
    Auth: AuthService,
{
    pub fn new(auth: Auth) -> Self {
        Self { auth }
    }
}

impl<Auth> Services for App<Auth>
where
    Auth: AuthService,
{
    type Auth = Auth;

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }
}
