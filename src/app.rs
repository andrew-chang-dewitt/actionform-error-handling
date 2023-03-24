use std::error::Error;

use leptos::{
    component, create_resource, create_rw_signal, create_server_action, server,
    server_fn::{self, ServerFn, ServerFnError},
    view, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};
use leptos_meta::*;
use leptos_router::{ActionForm, ActionFormProps, Router, RouterProps};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    pub handle: String,
    pub full_name: String,
    pub preferred_name: String,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start_axum.css"/>

        // sets the document title
        <Title text="Hoops | App"/>

        // app content
        <Router>
            <Register />
        </Router>
    }
}
/// Renders the registration page
#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let register_action = create_server_action::<Register>(cx);
    // FIXME: go back to using this impl
    // creating this signal and passing it to `ActionForm` via the `error` prop should work and is
    // probably the intended way to handle errors with `ActionForm`s, but somehow the RwSignal
    // isn't readable by function call or by it's `get()` method
    // let error = create_rw_signal::<Option<Box<dyn Error>>>(cx, None);
    // FIXME: this feels like a workaround, the above impl should be used when it can be made to
    // work correctly.
    // Instead, try reading the action's `value()` on each change
    // First, read the action as a resource to subscribe to changes
    let error = create_resource(
        cx,
        move || register_action.version().get(),
        move |_| async move {
            log::debug!("version changed!");
            let res = register_action.value().get()?;
            log::debug!("new value: {res:#?}");
            let as_option = res // value is an option, unwrap it to get the result contained
                .map_err(|e| Some(e.to_string()))
                .err()?; // process error to string, then return as some value
            log::debug!("as option: {as_option:#?}");

            as_option
        },
    );

    view! {
        cx,
        // <p>{error()}</p>
        // <ActionForm action=register_action error=error>
        <Suspense fallback={move || view!{cx, <p>"no errors?"</p>}}>
            <p>"error: "{move || -> Option<String> {
                let res = error.read(cx)?;
                log::debug!("Res inside suspense closure: {res:#?}");

                res
            }}</p>
        </Suspense>
        <ActionForm action=register_action>
            <label for="name">"name"</label>
            <input id="name" name="name" type="text" />
            <button type="submit">"Submit"</button>
        </ActionForm>
    }
}

#[server(Register, "/api")]
async fn register(_cx: Scope, name: String) -> Result<(), ServerFnError> {
    do_falliable_thing().await?;

    Ok(())
}

async fn do_falliable_thing() -> Result<(), ServerFnError> {
    Err(ServerFnError::ServerError(String::from("error!")))
}
