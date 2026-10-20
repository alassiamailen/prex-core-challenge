use crate::model::user_model::{Role, User};
use actix_web::HttpRequest;

/// Extract simulated user from request headers
///
/// Scenario:
/// Receive [X-User-Id], [X-Client-Id] and [X-Role] headers
/// Expectation:
/// A valid [User] should be returned
pub fn extract_user_from_headers(request: &HttpRequest) -> Result<User, ()> {
    // get user id header
    let user_id = request
        .headers()
        .get("X-User-Id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(())?;

    // get client id header
    let client_id = request
        .headers()
        .get("X-Client-Id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(())?;

    // get role header
    let role = request
        .headers()
        .get("X-Role")
        .and_then(|value| value.to_str().ok())
        .ok_or(())?;

    // map role header to application role
    let role = match role {
        "User" => Role::User,
        "SuperAdmin" => Role::SuperAdmin,
        _ => return Err(()),
    };

    Ok(User {
        user_id,
        client_id,
        role,
    })
}
