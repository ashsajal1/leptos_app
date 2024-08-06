use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ContactParams {
    id: Option<usize>
}

#[component]
pub fn User() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or_default()
        })
    };
    view! {
        <div>
            <h1>{id}</h1>
        </div>
    }
}
