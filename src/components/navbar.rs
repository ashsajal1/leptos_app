use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="p-2 bg-gray-800 navbar flex justify-between items-center">
            <h3>"Leptos App"</h3>
            <ul calss="flex items-center gap-2">
                <li class="btn btn-primary mr-2"><a href="/">Home</a></li>
                <li class="btn btn-primary mr-2"><a href="/about">About</a></li>
                <li class="btn btn-primary mr-2"><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    }
}
