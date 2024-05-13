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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    // insert_users(&pool, vec!["user 1".into(), "user 2".into()]).await?;
    // insert_currency(&pool).await?;
    let c_id = get_currency_id(&pool).await?;
    insert_expenses(&pool, vec![("cool expense".into(), 200, c_id)]).await?;
    // println!("{:?}", get_users(&pool).await.unwrap());
    Ok(())
}
