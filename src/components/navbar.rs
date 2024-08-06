use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="p-2 bg-gray-800">
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    }
}