use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// This component has a `#[cfg]`-gated element that only renders in
/// SSR. The WASM hydrate build doesn't see it, causing a DOM tree
/// mismatch during hydration → panic.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hydration Mismatch Demo"</h1>

        // This element only exists in the SSR HTML.
        // The WASM hydrate build skips it, so the DOM tree diverges.
        {#[cfg(feature = "ssr")]
        {
            view! { <p>"I only exist in SSR"</p> }.into_any()
        }}

        <p>"This paragraph exists in both SSR and WASM."</p>
        <p>"Open the browser console to see the panic."</p>
    }
}
