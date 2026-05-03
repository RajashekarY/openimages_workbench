use gpui::*;
use std::sync::Arc;

mod app;
mod models;
mod services;
mod ui;
mod utils;

use app::OpenImagesApp;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let app = OpenImagesApp::new(cx);
        cx.new_view(|cx| app)
            .map(|view| {
                cx.activate_window();
                
                // Set app title
                cx.open_window(
                    Default::default(),
                    |cx| {
                        cx.new_view(|_| view.clone())
                    },
                )
            })
            .ok();
    });
}