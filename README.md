# Leptos 0.8 Hydration Mismatch Panic

Minimal reproduction of a hydration panic when the SSR HTML tree differs from what the WASM hydrate build expects.

## Problem

When SSR renders a DOM element that the WASM hydrate build doesn't see (or vice versa), the app panics during hydration:

The page becomes completely non-functional — no error recovery, no fallback.

## Versions

- Leptos 0.8.17
- Rust stable (1.95.0)

## How to reproduce

### 1. Build and run

```bash
cargo leptos build
cargo leptos serve
```

### 2. Visit http://127.0.0.1:3000

### 3. Open browser console — observe the panic

The SSR HTML contains `<p>I only exist in SSR</p>`, but the WASM hydrate build doesn't know about it. When `tachys` walks the DOM to attach reactive behaviors, it expects the next sibling to be `<p>This paragraph exists in both SSR and WASM.</p>` but finds the extra `<p>` first. The `.unwrap()` on the missing expected node panics.

## Root cause

In `src/app.rs`, the `HomePage` component has a `#[cfg(feature = "ssr")]`-gated element:

```rust
{#[cfg(feature = "ssr")]
{
    view! { <p>"I only exist in SSR"</p> }.into_any()
}}
```

SSR compiles with `--features ssr`, so this element renders. WASM compiles with `--features hydrate`, so it's absent. The DOM trees diverge.

## This also happens when...

- `cargo leptos build` only recompiles one target (SSR binary changed but WASM was cached from a previous build)
- A reactive conditional evaluates differently between SSR's first render and hydration's first render
