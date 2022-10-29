
use {
    gloo_net::http,
    sycamore::{
        prelude::*,
        futures::*,
    },
};

fn main() {
    sycamore::render(|cx| {
        view!{cx,
            div(class="vbox layout-center") {
                Header {}
                div(id="body-column", class="vbox") {
                    Composer {}
                    Timeline {}
                }
            }
        }
    });
}

async fn send_toot(text: &str) -> Result<(), u16> {
    let resp = http::Request::post("/toot")
        .body(text)
        .send()
        .await
        .expect("send_toot");

    if resp.ok() { Ok(()) }
    else         { Err(resp.status()) }
}

#[component]
fn Composer<G: Html>(cx: Scope) -> View<G> {
    let text = create_signal(cx, String::new());
    let disable = create_signal(cx, false);
    let state = create_signal(cx, String::new());

    let click_send = move |_| {
        disable.set(true);
        state.set("Sending...".into());
        spawn_local_scoped(cx, async move {
            match send_toot(&text.get()).await {
                Ok(_) => {
                    state.set("Sent!".into());
                    text.set(String::new())
                }
                Err(status) => {
                    state.set(format!("Error {status}"));
                }
            }
            disable.set(false);
        });
    };

    view! {cx,
        div(id="composer", class="vbox") {
            textarea(class="composer", bind:value=text, prop:disabled=*disable.get())
            div(class="composer hbox") {
                output(class="composer") { (*state.get()) }
                button(class="composer", on:click=click_send, prop:disabled=*disable.get())
                    { "Toot!" }
            }
        }
    }
}

#[component]
fn Timeline<G: Html>(cx: Scope) -> View<G> {
    let items = vec![
        (100, "foo"),
        (101, "bar"),
        (102, "baz"),
    ];

    let items = create_signal(cx, items);

    view! {cx,
        Keyed(
            iterable = items,
            view = |cx, item| view!{cx,
                div(class="item") {
                    h1 { (item.1) }
                    p { (item.0) }
                }
            },
            key = |item| item.0,
        )
    }
}

#[component]
fn Header<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div(id="header", class="hbox") {
            img(class="logo", src="/assets/favicon.png")
            h1 { "CRUD Test" }
            /*SearchBar {}
            NavButtons {}*/
        }
    }
}
/*
#[component]
fn NavButtons<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div(class="nav hbox") {
            input(type="button", value="A")
            input(type="button", value="B")
            input(type="button", value="C")
            input(type="button", value="D")
        }
    }
}

#[component]
fn SearchBar<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div(class="search hbox") {
            input(type="text", class="search", placeholder="Search")
            input(type="button", value="🔍")
        }
    }
}
*/

