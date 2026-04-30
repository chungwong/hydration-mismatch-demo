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

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hydration Mismatch Demo"</h1>
        <p>"Hello from Leptos"</p>

        // STEP 2: Uncomment these two lines, then rebuild only SSR (not WASM).
        // <p>"This element was added after the initial build"</p>
        // <p>"The WASM doesn't know about it"</p>
    }
}
