use ::dioxus::prelude::*;
use ::rusty_casino::route::Route;

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

fn main() {
    asset!("../assets/dx-components-theme.css");
    LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
          ServeConfig::builder()
            // Enable incremental rendering
            .incremental(
              dioxus::server::IncrementalRendererConfig::new()
              // Store static files in the assets directory where other static
              // assets like wasm are stored
              .static_dir(
                std::env::current_exe()
                  .unwrap()
                  .parent()
                  .unwrap()
                  .join("assets")
              )
              // Don't clear the assets folder on every build. The assets folder
              // has other files including the wasm binary and static assets
              // required for the app to run
              .clear_cache(false)
            ).enable_out_of_order_streaming()
        })
        .launch(|| {
            rsx! {
              Router::<Route> {}
            }
        });
}
