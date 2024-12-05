use validator::Validate;

use crate::{App, AppError};

use super::{AuthService, SignInData, SignUpData, Token};

pub async fn sign_up<A: App>(data: SignUpData, app: &A) -> Result<Token, AppError> {
    data.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    app.auth().sign_up(data, app).await
}

pub async fn sign_in<A: App>(data: SignInData, app: &A) -> Result<Token, AppError> {
    data.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    app.auth().sign_in(data, app).await
}
