#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::LeptosRoutes;
    use hydration_mismatch_demo::app::App;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app = Router::new()
        .leptos_routes(&leptos_options, leptos_axum::generate_route_list(App), {
            let leptos_options = leptos_options.clone();
            move || hydration_mismatch_demo::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(hydration_mismatch_demo::shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
