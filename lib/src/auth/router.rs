use crate::{
    user::{NewUser, UserIdentifier},
    App, AppError,
};

use super::{AuthService, SignInData, SignUpData, Token};

pub async fn sign_up<A: App>(data: SignUpData, app: &A) -> Result<Token, AppError> {
    let new_user = NewUser {
        email: data.email,
        password_hash: app.auth().hash_password(data.password)?,
        name: data.name,
        photo_url: data.photo_url,
    };
    let user = app.user().create_user(new_user).await?;
    let token = app.auth().generate_token(user).await?;
    Ok(token)
}

pub async fn sign_in<A: App>(data: SignInData, app: &A) -> Result<Token, AppError> {
    let user = app
        .user()
        .get_user(UserIdentifier::new(None, Some(data.email))?)
        .await?;
    app.auth()
        .verify_password(data.password, &user.password_hash)?;
    app.auth().generate_token(user).await
}
