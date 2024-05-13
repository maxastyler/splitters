use std::fmt::Display;

use yew::{function_component, html, Html};

struct ExpenseUser {
    name: String,
}

impl Display for ExpenseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.name))
    }
}

struct Expense {
    amount: i32,
    users: Vec<ExpenseUser>,
}

#[function_component(ExpenseView)]
pub fn expense_view() -> Html {
    let data = Expense {
        amount: 100,
        users: vec![ExpenseUser { name: "hi".into() },
		    ExpenseUser {name: "ooh".into()}],
    };
    html! {
    <div><h1>{data.amount}</h1>
        <ul>
        {data.users.into_iter().map(|user| {
                html!{<li>{ format!("Hello, I'am {}!",user) }</li>}
        }).collect::<Html>()}
    </ul>
        </div>
    }
}
