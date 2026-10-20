use crate::errors::common_error::CommonError;
use crate::model::user_model::{Role, User};
use async_trait::async_trait;
use log::{debug, info};
use mockall::automock;
use std::sync::Arc;

/// Authorization service
#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthorizationServiceTrait {
    /// Validate if [User] is authorized to access [i32] client_id
    /// Returns a [CommonError] if the user is not authorized
    async fn authorize_client_access(&self, user: &User, client_id: i32)
        -> Result<(), CommonError>;

    /// Validate if [User] has [Role::SuperAdmin] permissions
    /// Returns a [CommonError] if the user is not a SuperAdmin
    async fn authorize_super_admin(&self, user: &User) -> Result<(), CommonError>;
}
/// Authorization service implementation struct
pub struct AuthorizationService;

/// Initialization
impl AuthorizationService {
    pub fn new() -> Self {
        AuthorizationService
    }
}
/// Authorization service implement logic
#[async_trait]
impl AuthorizationServiceTrait for AuthorizationService {
    /// Validate if [User] is authorized to access [i32] client_id
    /// Returns a [CommonError] if the user is not authorized
    async fn authorize_client_access(
        &self,
        user: &User,
        client_id: i32,
    ) -> Result<(), CommonError> {
        debug!("authorize_client_access - start");

        match user.role {
            Role::SuperAdmin => {
                info!("authorize_client_access - SuperAdmin done");
                Ok(())
            }
            Role::User => {
                if user.client_id == client_id {
                    info!("authorize_client_access - User done");
                    Ok(())
                } else {
                    Err(CommonError::Forbiden)
                }
            }
        }
    }

    /// Validate if [User] has [Role::SuperAdmin] permissions
    /// Returns a [CommonError] if the user is not a SuperAdmin
    async fn authorize_super_admin(&self, user: &User) -> Result<(), CommonError> {
        debug!("authorize_super_admin - start");

        match user.role {
            Role::SuperAdmin => {
                info!("authorize_super_admin - SuperAdmin authorized - done");
                Ok(())
            }
            Role::User => Err(CommonError::Forbiden),
        }
    }
}
/// Authorization service dyn type
pub type DynAuthorizationService = Arc<dyn AuthorizationServiceTrait + Send + Sync>;

/// Unit tests cases
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::automock;

    const USER_ID: i32 = i32::MAX;
    const USER_CLIENT_ID: i32 = i32::MAX - 1;
    const OTHER_CLIENT_ID: i32 = i32::MAX - 2;

    /// Scenario:
    /// Execute authorize_client_access when user accesses its own client
    /// Expectation:
    /// An Ok() should be returned
    #[tokio::test]
    async fn when_user_access_own_client_should_return_ok() {
        let user = User {
            user_id: USER_ID,
            client_id: USER_CLIENT_ID,
            role: Role::User,
        };

        let authorization_service = AuthorizationService::new();

        let expected_result = authorization_service
            .authorize_client_access(&user, USER_CLIENT_ID)
            .await;

        assert!(expected_result.is_ok());
    }

    /// Scenario:
    /// Execute authorize_client_access when user accesses another client
    /// Expectation:
    /// A [CommonError] should be returned
    #[tokio::test]
    async fn when_user_access_other_client_should_return_common_error() {
        let user = User {
            user_id: USER_ID,
            client_id: USER_CLIENT_ID,
            role: Role::User,
        };

        let authorization_service = AuthorizationService::new();

        let expected_result = authorization_service
            .authorize_client_access(&user, OTHER_CLIENT_ID)
            .await;

        assert_eq!(CommonError::Forbiden, expected_result.unwrap_err());
    }

    /// Scenario:
    /// Execute authorize_client_access when super admin accesses another client
    /// Expectation:
    /// An Ok() should be returned
    #[tokio::test]
    async fn when_super_admin_access_other_client_should_return_ok() {
        let user = User {
            user_id: USER_ID,
            client_id: USER_CLIENT_ID,
            role: Role::SuperAdmin,
        };

        let authorization_service = AuthorizationService::new();

        let expected_result = authorization_service
            .authorize_client_access(&user, OTHER_CLIENT_ID)
            .await;

        assert!(expected_result.is_ok());
    }
}
