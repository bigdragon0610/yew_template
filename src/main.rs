use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
      <h1 class="font-bold text-3xl mt-5 ml-10 p-5 bg-blue-100 table">{ "Hello Yew!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
