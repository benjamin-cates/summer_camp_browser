use home::Home;

pub mod camps;
pub mod structs;
pub mod html;
pub mod home;
pub mod search;

fn main() {
    yew::Renderer::<Home>::new().render();
}