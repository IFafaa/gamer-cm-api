use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::shared::{api_error::ApiErrorResponse, jwt_service::JwtService};

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, axum::Json<ApiErrorResponse>)> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiErrorResponse::new(
                    "Missing authorization header".to_string(),
                )),
            )
        })?;

    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            axum::Json(ApiErrorResponse::new(
                "Invalid authorization header format".to_string(),
            )),
        ));
    };

    let claims = JwtService::validate_token(token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            axum::Json(ApiErrorResponse::new(
                "Invalid or expired token".to_string(),
            )),
        )
    })?;

    let user_id = claims.sub.parse::<i32>().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            axum::Json(ApiErrorResponse::new(
                "Invalid user ID in token".to_string(),
            )),
        )
    })?;

    let user = AuthenticatedUser {
        id: user_id,
        username: claims.username,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
