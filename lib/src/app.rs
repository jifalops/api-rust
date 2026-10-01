use std::sync::Arc;

use crate::{auth::AuthService, user::UserService};

/// The composed application: one service per domain, already wired to its
/// adapters. Services are concrete rather than generic over their repository —
/// adapter choice happens once, in [`crate::init`], behind `Box<dyn ...Repo>`.
pub struct App {
    auth: Arc<AuthService>,
    user: Arc<UserService>,
}

impl App {
    pub fn new(auth: Arc<AuthService>, user: Arc<UserService>) -> Self {
        Self { auth, user }
    }

    pub fn auth(&self) -> &Arc<AuthService> {
        &self.auth
    }

    pub fn user(&self) -> &Arc<UserService> {
        &self.user
    }
}
