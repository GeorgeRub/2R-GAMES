use crate::TABLES_NAMES;
use crate::user::User;
use aws_sdk_dynamodb::types::AttributeValue;
use database::get_connection_to_db;

pub struct UserService;
pub trait UserServiceTrait {
    async fn is_user_exists(user_email: &str) -> bool;

    async fn get_user_by_email(user_email: &str) -> Option<User>;
    async fn save(user: &User) -> Result<User, String>;
}

impl UserServiceTrait for UserService {
    async fn is_user_exists(user_email: &str) -> bool {
        let client = get_connection_to_db().await;
        let data_table = TABLES_NAMES
            .lock()
            .unwrap()
            .get("USER_TABLE_NAME")
            .unwrap()
            .clone();
        let result = client
            .scan()
            .table_name(data_table)
            .filter_expression("user_email = :user_email ")
            .expression_attribute_values(
                ":user_email".to_string(),
                AttributeValue::S(user_email.to_string()),
            )
            .send()
            .await
            .unwrap();
        println!("RESULT ===>>>   {:?}", result);
        result.items.unwrap().len() > 0
    }

    async fn get_user_by_email(user_email: &str) -> Option<User> {
        let client = get_connection_to_db().await;
        let data_table = TABLES_NAMES
            .lock()
            .unwrap()
            .get("USER_TABLE_NAME")
            .unwrap()
            .clone();
        let result = client
            .scan()
            .table_name(data_table)
            .filter_expression("user_email = :user_email ")
            .expression_attribute_values(
                ":user_email".to_string(),
                AttributeValue::S(user_email.to_string()),
            )
            .send()
            .await;
        match result {
            Ok(us) => match us.items {
                Some(us_vec) => {
                    let user = User::deserialize_user(us_vec[0].clone());
                    Some(user)
                }
                None => return None,
            },
            Err(_) => return None,
        }
    }

    async fn save(user: &User) -> Result<User, String> {
        let is_user = Self::is_user_exists(&user.user_email).await;
        if is_user {
            return Err(format!(
                "User with email {} already exists",
                user.user_email
            ));
        }
        let client = get_connection_to_db().await;
        let data_table = TABLES_NAMES
            .lock()
            .unwrap()
            .get("USER_TABLE_NAME")
            .unwrap()
            .clone();

        let user_saved = client
            .put_item()
            .table_name(data_table)
            .set_item(Some(User::serialize_user(user.clone()).unwrap()))
            .send()
            .await;

        match user_saved {
            Ok(_) => Ok(user.clone()),
            Err(e) => Err(format!("Error saving user: {:?}", e)),
        }
    }
}
