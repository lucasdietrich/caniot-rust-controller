

use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>")]
pub fn web_hello(name: &str) -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}
