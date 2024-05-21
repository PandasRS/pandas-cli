#[cfg(test)]
mod tests {
    use crate::modules::users::dto::{CreateUsersDto, UpdateUsersDto};
    use crate::modules::users::service::*;
    use crate::modules::users::repository_mock::MockUsersRepository;

    #[tokio::test]
    async fn test_create_users_service() {
        let mut mock_repo = MockUsersRepository::new();
        let dto = CreateUsersDto {
            name: "New Users".to_string(),
            age: 5,
        };

        let result = create_users(&mut mock_repo, dto).await.into_inner();
        assert_eq!(result.name, "New Users");
        assert_eq!(result.age, 5);
        assert!(result.id.is_some());
    }

    #[tokio::test]
    async fn test_get_users_service() {
        let mut mock_repo = MockUsersRepository::new();
        mock_repo.insert_mock_users("1", "Users One", 3);
        mock_repo.insert_mock_users("2", "Users Two", 4);

        let result = get_users(&mut mock_repo).await.into_inner();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_users_by_id_service() {
      let mut mock_repo = MockUsersRepository::new();
      mock_repo.insert_mock_users(&"123".to_string(), "Single Users", 5);

      let result = get_users_by_id(&mut mock_repo, &"123".to_string()).await;
      assert!(result.is_some());
      let users = result.unwrap().into_inner();
      assert_eq!(users.name, "Single Users");
      assert_eq!(users.age, 5);
    }

    #[tokio::test]
    async fn test_update_users_service() {
        let mut mock_repo = MockUsersRepository::new();
        mock_repo.insert_mock_users("update_id", "Old Users", 7);

        let update_dto = UpdateUsersDto {
            name: Some("Updated Users".to_string()),
            age: Some(8),
        };

        let result = update_users(&mut mock_repo, "update_id", update_dto).await;
        assert!(result.is_some());
        let updated_users = result.unwrap().into_inner();
        assert_eq!(updated_users.name, "Updated Users");
        assert_eq!(updated_users.age, 8);
    }

    #[tokio::test]
    async fn test_delete_users_service() {
        let mut mock_repo = MockUsersRepository::new();
        mock_repo.insert_mock_users("delete_id", "Delete Users", 6);

        let result = delete_users(&mut mock_repo, "delete_id").await;
        assert!(result);
    }
}
