use crate::TABLES_NAMES;
use crate::user::User;
use aws_sdk_dynamodb::types::AttributeValue;
use database::get_connection_to_db;
use lambda_http::aws_lambda_events::query_map::QueryMap;
use std::cmp::Reverse;

pub struct UserService;
pub trait UserServiceTrait {
    async fn is_user_exists(user_email: &str) -> bool;

    async fn get_user_by_email(user_email: &str) -> Option<User>;
    async fn save(user: &User) -> Result<User, String>;

    async fn get_all_users(pagination: QueryMap) -> Vec<User>;
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

    async fn get_all_users(pagination: QueryMap) -> Vec<User> {
        let page = match pagination.first("page") {
            Some(page) => page.parse::<i32>().unwrap(),
            None => 1,
        };
        let size = match pagination.first("size") {
            Some(size) => size.parse::<i32>().unwrap(),
            None => 10,
        };
        let sort: String = match pagination.first("sort") {
            Some(sort) => sort.to_string().split(",").collect::<Vec<&str>>()[0].to_string(),
            None => "user_id".to_string(),
        };
        let sort_direction = match pagination.first("sortDirection") {
            Some(sort_direction) => sort_direction.to_string(),
            None => "ASC".to_string(),
        };

        let client = get_connection_to_db().await;
        let data_table = TABLES_NAMES
            .lock()
            .unwrap()
            .get("USER_TABLE_NAME")
            .unwrap()
            .clone();

        println!(
            "SELECT * FROM {:?} ORDER BY {} {}",
            data_table, sort, sort_direction
        );

        let sql = format!(
            r#"SELECT * FROM "{}" WHERE {} <> ? ORDER BY {}"#,
            data_table, sort, sort
        );
        let scan_result = client
            .execute_statement()
            .statement(sql)
            .limit(size)
            .send()
            .await;

        // let scan_result = client
        //     .query()
        //     .table_name(&data_table)
        //     // .index_name("user_id") // Replace with actual GSI name
        //     .key_condition_expression(format!(
        //         "select * from {:?} WHERE {} not in [1]",
        //         &data_table, sort
        //     )) // GSI partition key
        //     // .filter_expression("attribute_exists(#user_id)")
        //     // .expression_attribute_values(":gsi_pk", AttributeValue::S("ALL_USERS".to_string())) // Constant value
        //     // .expression_attribute_names("#user_id", "user_id")
        //     // .filter_expression("attribute_exists(user_id)")
        //     // .expression_attribute_names("#user_id", "user_id")
        //     .scan_index_forward(sort_direction == "asc")
        //     .limit(size)
        //     .send()
        //     .await;
        // let scan_result = client
        //     .scan()
        //     .table_name(data_table)
        //     .filter_expression("attribute_exists(#sort_item)")
        //     .expression_attribute_names("#sort_item", format!("{}", sort))
        //     // .scan_index_forward(sort_direction == "asc")
        //     .limit(size)
        //     .send()
        //     .await;
        // let scan_result = client
        //     .execute_statement()
        //     .statement(format!(
        //         "SELECT * FROM {:?} ORDER BY {} {}",
        //         data_table, sort, sort_direction
        //     ))
        //     // .key_condition_expression(format!("#{} = :{}", sort, sort))
        //     // .expression_attribute_names(format!("#{}", sort), format!("{}", sort))
        //     // .scan_index_forward(sort_direction == "asc")
        //     .limit(size)
        //     .send()
        //     .await;

        match scan_result {
            Ok(us) => {
                us.items
                    .unwrap()
                    .iter()
                    .map(|u| User::deserialize_user(u.clone()))
                    .collect()
                // let mut list_of_users: Vec<User> = items
                //     .iter()
                //     .map(|u| User::deserialize_user(u.clone()))
                //     .collect();
                // if sort_direction == "asc" {
                //     println!("SORTING ASC");
                //     list_of_users.sort_by_key(|user| {
                //         user.user_first_name.clone()
                //     })
                // } else if sort_direction == "desc" {
                //     println!("SORTING DESC");
                //     list_of_users.sort_by_key(|user| {
                //         Reverse(user.user_last_name.clone())
                //     })
                // }
                // list_of_users
            }
            Err(error) => {
                println!("ERROR: {:?}", error);
                vec![]
            }
        }
    }
}
