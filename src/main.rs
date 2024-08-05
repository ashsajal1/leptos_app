mod components;
use components::navbar::Navbar;
use components::counter::Counter;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Navbar />
            <main>
                <h1>"Welcome to My App!"</h1>
            </main>
        </div>

        <Counter initial_value=5 />
    }
}

fn main() {
    mount_to_body(App);
}