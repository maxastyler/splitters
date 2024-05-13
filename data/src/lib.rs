use serde::{Serialize, Deserialize};
use serde_json::Result as JSONResult;
use sqlx::Type;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/expense/:id/")]
    Expense { id: i32 },
}


#[derive(Serialize, Deserialize, Debug, Type)]
pub struct ExpenseUser {
    pub id: i32,
    pub name: String,
    pub proportion_owed: i64,
    pub paid: i64
}

#[derive(Serialize, Deserialize)]
pub struct Expense {
    pub amount: i64,
    pub description: String,
    pub users: Vec<ExpenseUser>
}

#[cfg(test)]
mod tests {
    use super::*;
}
