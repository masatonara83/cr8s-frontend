use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::crates::api_crate_create;
use crate::api::crates::api_crate_update;
use crate::api::crates::Crate;
use crate::components::button::Button;
use crate::Route;

use crate::components::alert::Alert;
use crate::components::alert::AlertType;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub a_crate: Option<Crate>,
}

#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    //リダイレクトする
    let navigator = use_navigator().expect("Navigator not available");
    //コンテキストを取得
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context id missing");

    //name用ハンドル
    let name_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.name.clone();
        }
        String::default()
    });
    let name = (*name_handle).clone();

    //code用ハンドル
    let code_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.code.clone();
        }
        String::default()
    });
    let code = (*code_handle).clone();

    //
    let version_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.version.clone();
        }
        String::default()
    });
    let version = (*version_handle).clone();

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let code_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            code_handle.set(input.value());
        }
    });

    let version_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            version_handle.set(input.value());
        }
    });

    let cloned_name = name.clone();
    let cloned_code = code.clone();
    let cloned_crate: Option<Crate> = props.a_crate.clone();

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_name = cloned_name.clone();
        let cloned_code = cloned_code.clone();
        let cloned_crate = cloned_crate.clone();

        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = current_user_ctx.clone();
        match &cloned_user_ctx.token {
            Some(token) => {
                let cloned_token = token.clone();

                spawn_local(async move {
                    if let Some(a_crate) = cloned_crate {
                        match api_crate_update(
                            &cloned_token,
                            a_crate.id.clone(),
                            cloned_name.clone(),
                            cloned_code.clone(),
                        )
                        .await
                        {
                            Ok(_) => cloned_navigator.push(&Route::Crates),
                            Err(e) => cloned_error_handle.set(e.to_string()),
                        }
                    } else {
                        match api_crate_create(
                            &cloned_token,
                            cloned_name.clone(),
                            cloned_code.clone(),
                        )
                        .await
                        {
                            Ok(_) => cloned_navigator.push(&Route::Crates),
                            Err(e) => cloned_error_handle.set(e.to_string()),
                        }
                    }
                });
            }
            None => cloned_error_handle.set("Session expired. Please login again".to_string()),
        }
    });

    html! {
      <form onsubmit={onsubmit}>
        if error_message.len() > 0 {
          < Alert
            alert_type={AlertType::Danger}
            message={error_message}
          />
        }
        <div class="mb-3">
          <Input
            input_type="text"
            name="code"
            label="Code"
            value={code}
            onchange={code_changed}
          />
        </div>
        <div class="mb-3">
          <Input
            input_type="text"
            name="name"
            label="Name"
            value={name}
            onchange={name_changed}
          />
        </div>
        <div class="mb-3">
          <Input
            input_type="text"
            name="version"
            label="Version"
            value={version}
            onchange={version_changed}
          />
        </div>
          <Button button_type="primary" label="Save" />
      </form>
    }
}
