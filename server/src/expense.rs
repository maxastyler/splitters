use axum::{extract::Path, http::Response, response::IntoResponse, Json};
use sqlx::{query, PgPool, Pool};

use anyhow::Result;
use data::{Expense, ExpenseUser};

use crate::State;
use axum::extract::State as ExtractState;

async fn get_expense(pool: &PgPool, id: i32) -> Result<Expense> {
    let e = query!(
        r#"
SELECT e.description, e.amount, ue.proportion_owed, ue.amount_paid, u.username, u.id
FROM expenses e
LEFT JOIN user_to_expense ue ON ue.expense_id = e.id
JOIN users u ON ue.user_id = u.id
WHERE e.id = $1
"#,
        id
    )
    .fetch_all(pool)
    .await?;
    let users = e
        .iter()
        .map(|r| ExpenseUser {
            id: r.id,
            name: r.username.clone(),
            proportion_owed: r.proportion_owed,
            paid: r.amount_paid.unwrap_or(0),
        })
        .collect::<Vec<_>>();
    let expense = e.into_iter().next().expect("oops");

    Ok(Expense {
        amount: expense.amount,
        description: expense.description.unwrap_or("".into()),
        users,
    })
}

pub async fn get_expense_json(
    Path(id): Path<i32>,
    ExtractState(State { pool }): ExtractState<State>,
) -> impl IntoResponse {
    //     let res = query!(r#"SELECT expenses.description, expenses.amount,
    // ARRAY_AGG((users.username, users.id, user_to_expense.proportion_owed, user_to_expense.amount_paid)) as "users!: Vec<ExpenseUser>"
    // FROM expenses JOIN user_to_expense ON user_to_expense.expense_id = expenses.id JOIN users ON users.id = user_id WHERE expenses.id = $1"#, id);
    //     query!(r#"
    // SELECT expenses.description, expenses.amount FROM expenses WHERE expenses.id = $1 LIMIT 1
    // "#, id).fetch_one()
    let expense = get_expense(&pool, id).await;
    Json(expense.unwrap())
}
