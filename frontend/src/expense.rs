use std::fmt::Display;

use data::Expense;
use gloo_net::http::Request;
use yew::{function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle};

struct ExpenseUser {
    name: String,
}

impl Display for ExpenseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.name))
    }
}

enum ExpenseViewState {
    Loading,
    Error,
    Got(Expense),
}

#[derive(Properties, PartialEq)]
pub struct ExpenseViewProps {
    pub id: i32,
}

#[function_component(ExpenseView)]
pub fn expense_view(&ExpenseViewProps { id }: &ExpenseViewProps) -> Html {
    let expense: UseStateHandle<ExpenseViewState> = use_state(|| ExpenseViewState::Loading);
    {
        let expense = expense.clone();
        use_effect_with(id, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(&format!("/api/expense/{}", id)).send().await {
                    Ok(response) => {
                        expense.set(ExpenseViewState::Got(response.json().await.unwrap()))
                    }
                    Err(_) => expense.set(ExpenseViewState::Error),
                };
            })
        })
    }

    match &(*expense) {
        ExpenseViewState::Got(x) => html! {
            <div>
        <h1>{&x.description}</h1>
        <h2>{x.amount}</h2>
            <ul>
            {x.users.iter().map(|user| {
                    html!{<li>{ format!("Hello, I'am {}!",user.name) }</li>}
            }).collect::<Html>()}
        </ul>
            </div>
        },
        ExpenseViewState::Loading => html! {<div> {"Loading expense...."}</div>},
        ExpenseViewState::Error => html! {<div>{"No expense "} {id}</div>},
    }
}
