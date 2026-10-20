/// Role Enum
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    SuperAdmin,
}
/// User model
#[derive(Debug, Clone)]
pub struct User {
    // user id
    pub user_id: i32,
    // client id associated with the user
    pub client_id: i32,
    // user role
    pub role: Role,
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use super::*;

    const USER_ID: i32 = i32::MAX;
    const CLIENT_ID: i32 = i32::MAX - 1;

    /// Get User model
    #[tokio::test]
    async fn test_user_model() {
        let role = Role::User;

        let expected_result = User {
            user_id: USER_ID,
            client_id: CLIENT_ID,
            role: role.clone(),
        };

        assert_eq!(USER_ID, expected_result.user_id);
        assert_eq!(CLIENT_ID, expected_result.client_id);
        assert_eq!(role, expected_result.role);
    }
    /// Get SuperAdmin role
    #[tokio::test]
    async fn test_super_admin_role() {
        let expected_result = Role::SuperAdmin;

        assert_eq!(Role::SuperAdmin, expected_result);
    }

    /// Get User role
    #[tokio::test]
    async fn test_user_role() {
        let expected_result = Role::User;

        assert_eq!(Role::User, expected_result);
    }
}
