use crate::http_communication::pagination::Pagination;
use crate::user::User;
use sqlx::{Connection, Error, Pool, Postgres, QueryBuilder};

pub struct UserRepository {}

impl UserRepository {
    pub async fn get_all_users(
        connection: &Pool<Postgres>,
        pagination: Pagination,
    ) -> Result<Vec<User>, Error> {
        println!("Pagination: {:?}", pagination);
        let mut sql = QueryBuilder::<Postgres>::new("SELECT * FROM users");
        sql.push(" ORDER BY ");
        sql.push(pagination.sort);
        sql.push(" ");
        sql.push(pagination.sort_direction);
        sql.push(" LIMIT ");
        sql.push(pagination.size);
        sql.push(" OFFSET ");
        sql.push(pagination.offset);
        let query = sql.build_query_as::<User>();
        query.fetch_all(connection).await
    }

    pub async fn get_user_by_email(connection: &Pool<Postgres>, user_email: &str) -> Option<User> {
        let mut sql = QueryBuilder::<Postgres>::new("SELECT * FROM users");
        sql.push(" WHERE user_email = ");
        sql.push(user_email);
        let query = sql.build_query_as::<User>();
        query.fetch_one(connection).await.ok()
    }

    //Creating a new record in the database
    pub async fn save(connection: &Pool<Postgres>, user: &User) -> Result<(), Error> {
        //TODO: save user

        if Self::get_user_by_email(connection, &user.user_email)
            .await
            .is_some()
        {
            return Err(Error::Protocol(format!(
                "User with email {} already exists",
                user.user_email
            )));
        }

        let mut qb = QueryBuilder::new(
            "INSERT INTO users (user_email, doc_id, user_first_name, user_last_name, user_active) ",
        );

        qb.push("VALUES (");
        qb.push_bind(user.user_email.as_str()).push(", ");
        qb.push_bind(&user.doc_id).push(", ");
        qb.push_bind(user.user_first_name.as_str()).push(", ");
        qb.push_bind(user.user_last_name.as_str()).push(", ");
        qb.push(user.user_active).push(");");

        let qb_build = qb.build();

        let mut tx = connection.begin().await?;
        let res = qb_build.execute(&mut *tx).await;
        match res {
            Ok(_) => println!("User saved"),
            Err(e) => println!("===>>>    Error saving user {:?}", e),
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn save_all(connection: &Pool<Postgres>, users: Vec<User>) {
        //TODO: save users
    }

    pub async fn update(connection: &Pool<Postgres>, user: &User) {
        //TODO: update user
    }

    pub async fn update_all(connection: &Pool<Postgres>, users: Vec<User>) {
        //TODO: update users
    }

    pub async fn delete(connection: &Pool<Postgres>, user: &User) {
        //TODO: delete user
    }

    pub async fn delete_all(connection: &Pool<Postgres>, user: &User) {
        //TODO: delete users
    }
}
