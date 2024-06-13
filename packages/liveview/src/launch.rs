use dioxus_core::*;
use std::any::Any;

pub type Config = crate::Config<axum::Router>;

fn lazy_tokio_runtime() -> tokio::runtime::Handle {
    if let Ok(current) = tokio::runtime::Handle::try_current() {
        return current;
    }

    #[cfg(feature = "multi-threaded")]
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    #[cfg(not(feature = "multi-threaded"))]
    let mut builder = tokio::runtime::Builder::new_current_thread();

    builder.enable_all().build().unwrap().handle().clone()
}

/// Launches the WebView and runs the event loop, with configuration and root props.
pub fn launch(
    root: fn() -> Element,
    contexts: Vec<Box<dyn Fn() -> Box<dyn Any> + Send + Sync>>,
    platform_config: Config,
) {
    lazy_tokio_runtime().block_on(async move {
        platform_config
            .with_virtual_dom(move || {
                let mut virtual_dom = VirtualDom::new(root);

                for context in &contexts {
                    virtual_dom.insert_any_root_context(context());
                }

                virtual_dom
            })
            .launch()
            .await;
    });
}
