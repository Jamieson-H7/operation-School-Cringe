use std::sync::Arc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    HtmlButtonElement, HtmlDivElement, HtmlInputElement, HtmlParagraphElement, Response,
};

struct App {
    input: Mutable<String>,
    output: Mutable<String>,
}

impl App {
    fn new() -> Arc<App> {
        Arc::new(App {
            input: Mutable::new(String::new()),
            output: Mutable::new(String::new()),
        })
    }
    fn render_main(app: Arc<App>) -> Dom {
        html!("div" => HtmlDivElement, {
            .class("container")
            .style("margin-top", "10%")

            .children(&mut [
                html!("label", {
                    .attr("for", "urlInput")
                    .text("URL to fetch")
                }),

                html!("input" => HtmlInputElement, {
                    .attr("id", "urlInput")
                    .class("u-full-width")
                    .attr("type", "text")

                    .with_node!(element => {
                        .event(clone!(app => move |_: events::Input| {
                            app.input.set_neq(element.value());
                        }))
                    })
                }),

                html!("button" => HtmlButtonElement, {
                    .class("u-full-width")
                    .attr("id", "submitUrl")
                    .text("Submit request")

                    .event(clone!(app => move |_: events::Click| {
                        let url = format!("https://api.codetabs.com/v1/proxy/?quest={}", app.input.get_cloned());
                        let output = app.output.clone();
                        spawn_local(async move {
                            let future = web_sys::window().unwrap()
                                .fetch_with_str(&url);

                            let response = JsFuture::from(future)
                                .await
                                .unwrap()
                                .unchecked_into::<Response>();

                            let value = JsFuture::from(response.text().unwrap())
                                .await
                                .unwrap()
                                .as_string()
                                .unwrap();

                            output.set_neq(value);
                        })
                    }))
                }),

                html!("p" => HtmlParagraphElement, {
                    .text_signal(app.output.signal_cloned())
                })
            ])
        })
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render_main(app));
}
