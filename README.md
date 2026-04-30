# Leptos 0.8 Hydration Mismatch Panic

Minimal reproduction of a hydration panic caused by SSR and WASM builds being out of sync.

## How to reproduce

### Step 1: Build both targets and verify the app works

```bash
cargo leptos build
cargo leptos serve
```

Visit http://127.0.0.1:3000 — the page renders correctly. Stop the server.

### Step 2: Change the view structure

In `src/app.rs`, uncomment the two `<p>` lines:

```rust
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hydration Mismatch Demo"</h1>
        <p>"Hello from Leptos"</p>

        // Uncomment these:
        <p>"This element was added after the initial build"</p>
        <p>"The WASM doesn't know about it"</p>
    }
}
```

### Step 3: Rebuild ONLY the SSR binary (not WASM)

```bash
cargo leptos build
```
