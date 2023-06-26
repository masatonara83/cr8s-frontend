use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::sidebar::Sidebar, contexts::CurrentUserContext, Route};

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context id missing");
    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="container">
                    <div class="row">
                      <div class="col">
                        <Sidebar />
                      </div>
                      <div class="col">
                        {"Welcome user "}{user.username.clone()}
                      </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}
