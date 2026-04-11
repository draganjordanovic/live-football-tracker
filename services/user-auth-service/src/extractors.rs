use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{decode_jwt, Claims},
};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = Arc::<AppState>::from_ref(state);

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Authorization header".to_string()))?;

        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid bearer token format".to_string()))?;

        let claims: Claims = decode_jwt(token, &state.jwt_secret)
            .map_err(|e| {
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Invalid token: {}", e),
                )
            })?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token subject".to_string()))?;

        Ok(AuthenticatedUser {
            user_id,
            email: claims.email,
            role: claims.role,
        })
    }
}