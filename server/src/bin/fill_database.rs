use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgQueryResult, PgRow},
    query::Query,
};
use std::env::var;

async fn insert_users(pool: &PgPool, users: Vec<String>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users(username) SELECT * FROM UNNEST($1::text[])",
        &users[..]
    )
    .execute(pool)
    .await
}

async fn insert_currency(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"INSERT INTO currencies(name) VALUES ('pounds')"#,)
        .execute(pool)
        .await?;
    Ok(())
}

async fn get_currency_id(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query!(r#"SELECT id FROM currencies LIMIT 1"#)
        .fetch_one(pool)
        .await
        .map(|r| r.id as i64)
}

async fn get_user_ids(pool: &PgPool) -> Result<Vec<i32>, sqlx::Error> {
    let res = sqlx::query!("SELECT id FROM users").fetch_all(pool).await?;
    Ok(res.into_iter().map(|x| x.id).collect::<Vec<_>>())
}

async fn insert_expenses(
    pool: &PgPool,
    expenses: Vec<(String, i64, i64)>,
) -> Result<(), sqlx::Error> {
    let mut vecs = (
        Vec::with_capacity(expenses.len()),
        Vec::with_capacity(expenses.len()),
        Vec::with_capacity(expenses.len()),
    );
    for (a, b, c) in expenses.into_iter() {
        vecs.0.push(a);
        vecs.1.push(b);
        vecs.2.push(c);
    }
    sqlx::query!(
        r#"INSERT INTO expenses(description, amount, currency_id) SELECT * FROM UNNEST($1::text[], $2::bigint[], $3::bigint[])"#,
        &vecs.0[..], &vecs.1[..], &vecs.2[..]
    ).execute(pool).await?;
    Ok(())
}

async fn connect_users_and_expenses(pool: &PgPool) -> Result<(), sqlx::Error> {
    let user_ids = get_user_ids(pool).await?;
    let expense_ids = sqlx::query!(r#"SELECT id FROM expenses"#)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();
    sqlx::query!(
        r#"INSERT INTO user_to_expense(user_id, expense_id, proportion_owed, amount_paid)
SELECT * FROM UNNEST($1::int[], $2::int[], $3::bigint[], $4::bigint[])"#,
        &vec![user_ids[0], user_ids[1]],
        &vec![expense_ids[0], expense_ids[1]],
	&vec![0; 2],
	&vec![2; 2]
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn delete_all(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"DELETE FROM user_to_expense"#).execute(pool).await?;
    sqlx::query!(r#"DELETE FROM expenses"#).execute(pool).await?;
    sqlx::query!(r#"DELETE FROM currencies"#).execute(pool).await?;    
    sqlx::query!(r#"DELETE FROM friendships"#).execute(pool).await?;
    sqlx::query!(r#"DELETE FROM users"#).execute(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    delete_all(&pool).await?;
    insert_users(&pool, vec!["user 1".into(), "user 2".into()]).await?;
    insert_currency(&pool).await?;
    let c_id = get_currency_id(&pool).await?;
    insert_expenses(&pool, vec![("cool expense".into(), 200, c_id),
				("other expense".into(), 200, c_id)]).await?;
    connect_users_and_expenses(&pool).await?;
    // println!("{:?}", get_users(&pool).await.unwrap());
    Ok(())
}
