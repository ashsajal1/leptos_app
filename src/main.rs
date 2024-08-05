mod components;
use components::navbar::Navbar;
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
    }
}

fn main() {
    mount_to_body(App);
}