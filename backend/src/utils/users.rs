use sqlx::Row;

#[tracing::instrument(name = "Getting an active user from the DB.", skip(pool))]
pub async fn get_active_user_from_db(
    pool: Option<&sqlx::postgres::PgPool>,
    transaction: Option<&mut sqlx::Transaction<'_, sqlx::Postgres>>,
    id: Option<uuid::Uuid>,
    email: Option<&String>,
) -> Result<crate::types::User, sqlx::Error> {
    let mut query_builder =
        sqlx::query_builder::QueryBuilder::new(crate::queries::USER_AND_USER_PROFILE_QUERY);

    if let Some(id) = id {
        query_builder.push(" u.id=");
        query_builder.push_bind(id);
    }

    if let Some(e) = email {
        query_builder.push(" u.email=");
        query_builder.push_bind(e);
    }

    let sqlx_query = query_builder
        .build()
        .map(|row: sqlx::postgres::PgRow| crate::types::User {
            id: row.get("u_id"),
            email: row.get("u_email"),
            first_name: row.get("u_first_name"),
            password: row.get("u_password"),
            last_name: row.get("u_last_name"),
            is_active: row.get("u_is_active"),
            is_staff: row.get("u_is_staff"),
            is_superuser: row.get("u_is_superuser"),
            thumbnail: row.get("u_thumbnail"),
            date_joined: row.get("u_date_joined"),
            profile: crate::types::UserProfile {
                id: row.get("p_id"),
                user_id: row.get("p_user_id"),
                phone_number: row.get("p_phone_number"),
                birth_date: row.get("p_birth_date"),
                github_link: row.get("p_github_link"),
            },
        });

    let fetched_query = {
        if pool.is_some() {
            let p = pool.unwrap();
            sqlx_query.fetch_one(p).await
        } else {
            let t = transaction.unwrap();
            sqlx_query.fetch_one(&mut *t).await
        }
    };
    match fetched_query {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}
