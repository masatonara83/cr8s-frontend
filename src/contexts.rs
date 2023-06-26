use std::rc::Rc;

use gloo_storage::SessionStorage;
use gloo_storage::Storage;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

use crate::api::user::api_me;
use crate::api::user::{LoginResponse, MeResponse, User};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess => {
                let me_res = action.me_response.expect("Missing me response");
                let login_res = action.login_response.expect("Missing login response");
                let _ = SessionStorage::set("cr8s_token", login_res.token.clone());
                Self {
                    user: Some(User {
                        id: me_res.id,
                        username: me_res.username,
                        created_at: me_res.created_at,
                    }),
                    token: Some(login_res.token),
                }
                .into()
            }
            CurrentUserActions::LoginFail => {
                SessionStorage::clear();
                Self {
                    user: None,
                    token: None,
                }
            }
            .into(),
        }
    }
}

pub struct CurrentUserDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);
    if user.user.is_none() {
        if let Ok(token) = SessionStorage::get::<String>("cr8s_token") {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => cloned_user.dispatch(CurrentUserDispatchActions {
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(LoginResponse { token }),
                        me_response: Some(me_response),
                    }),
                    Err(_) => SessionStorage::clear(),
                }
            });
        }
    }

    html! {
      <ContextProvider<CurrentUserContext> context = {user}>
            {props.children.clone()}
      </ContextProvider<CurrentUserContext>>
    }
}
