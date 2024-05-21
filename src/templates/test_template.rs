#[cfg(test)]
mod tests {
    use crate::modules::{{module_name}}::dto::{Create{{ModuleName}}Dto, Update{{ModuleName}}Dto};
    use crate::modules::{{module_name}}::service::*;
    use crate::modules::{{module_name}}::repository_mock::Mock{{ModuleName}}Repository;

    #[tokio::test]
    async fn test_create_{{module_name}}_service() {
        let mut mock_repo = Mock{{ModuleName}}Repository::new();
        let dto = Create{{ModuleName}}Dto {
            name: "New {{ModuleName}}".to_string(),
            age: 5,
        };

        let result = create_{{module_name}}(&mut mock_repo, dto).await.into_inner();
        assert_eq!(result.name, "New {{ModuleName}}");
        assert_eq!(result.age, 5);
        assert!(result.id.is_some());
    }

    #[tokio::test]
    async fn test_get_{{module_name}}_service() {
        let mut mock_repo = Mock{{ModuleName}}Repository::new();
        mock_repo.insert_mock_{{module_name}}("1", "{{ModuleName}} One", 3);
        mock_repo.insert_mock_{{module_name}}("2", "{{ModuleName}} Two", 4);

        let result = get_{{module_name}}(&mut mock_repo).await.into_inner();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_{{module_name}}_by_id_service() {
      let mut mock_repo = Mock{{ModuleName}}Repository::new();
      mock_repo.insert_mock_{{module_name}}(&"123".to_string(), "Single {{ModuleName}}", 5);

      let result = get_{{module_name}}_by_id(&mut mock_repo, &"123".to_string()).await;
      assert!(result.is_some());
      let {{module_name}} = result.unwrap().into_inner();
      assert_eq!({{module_name}}.name, "Single {{ModuleName}}");
      assert_eq!({{module_name}}.age, 5);
    }

    #[tokio::test]
    async fn test_update_{{module_name}}_service() {
        let mut mock_repo = Mock{{ModuleName}}Repository::new();
        mock_repo.insert_mock_{{module_name}}("update_id", "Old {{ModuleName}}", 7);

        let update_dto = Update{{ModuleName}}Dto {
            name: Some("Updated {{ModuleName}}".to_string()),
            age: Some(8),
        };

        let result = update_{{module_name}}(&mut mock_repo, "update_id", update_dto).await;
        assert!(result.is_some());
        let updated_{{module_name}} = result.unwrap().into_inner();
        assert_eq!(updated_{{module_name}}.name, "Updated {{ModuleName}}");
        assert_eq!(updated_{{module_name}}.age, 8);
    }

    #[tokio::test]
    async fn test_delete_{{module_name}}_service() {
        let mut mock_repo = Mock{{ModuleName}}Repository::new();
        mock_repo.insert_mock_{{module_name}}("delete_id", "Delete {{ModuleName}}", 6);

        let result = delete_{{module_name}}(&mut mock_repo, "delete_id").await;
        assert!(result);
    }
}
