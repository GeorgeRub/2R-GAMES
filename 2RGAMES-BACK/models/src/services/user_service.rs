use crate::http_communication::pagination::Pagination;
use crate::repositories::user_repository::UserRepository;
use crate::user::User;
use database::get_connection_to_db;

pub struct UserService;

impl UserService {
    pub async fn is_user_exists(user_email: &str) -> bool {
        let user = Self::get_user_by_email(user_email).await;
        match user {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn get_user_by_email(user_email: &str) -> Option<User> {
        let client = get_connection_to_db().await;
        UserRepository::get_user_by_email(&client, user_email).await
    }

    pub async fn save(user: &User) -> Result<(), String> {
        let client = get_connection_to_db().await;
        let saved_user = UserRepository::save(&client, user).await;
        match saved_user {
            Ok(_) => {Ok(())},
            Err(_) => Err("Service === >>> Error saving user".to_string()),
        }
    }

    pub async fn get_all_users(pagination: Pagination) -> Vec<User> {
        let client = get_connection_to_db().await;
        let resp = UserRepository::get_all_users(&client, pagination).await;
        println!("{:?}", resp);
        resp.unwrap_or_else(|_| vec![])
    }
}
