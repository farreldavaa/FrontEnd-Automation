use yew::prelude::*;
use yew_router::prelude::*;


mod components;
mod pages;
mod router;
mod app;
mod store;
mod types;


use app::App;



fn main() {
    yew::start_app::<App>();
}