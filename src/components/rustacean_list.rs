use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::rustaceans::api_rustaceans;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(RustaceanList)]
pub fn rustacean_list() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context id missing");

    let rustaceans_handle = use_state(|| vec![]);
    let rustaceans = (*rustaceans_handle).clone();

    match &current_user_ctx.token {
        Some(token) => {
            let cloned_token = token.clone();
            spawn_local(async move {
                let response = api_rustaceans(&cloned_token).await.unwrap();
                rustaceans_handle.set(response);
            });

            html! {
              <>
                <p>
                  <Link<Route> to={Route::RustaceansAdd}>
                    {"+ Add new rustacean"}
                  </Link<Route>>
                </p>
                <table class="table">
                  <thead>
                    <tr>
                      <th>{"ID"}</th>
                      <th>{"Name"}</th>
                      <th>{"Email"}</th>
                      <th>{"Created at"}</th>
                      <th>{"Operations"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {
                      rustaceans.into_iter().map(|r| {
                          html! {
                            <tr>
                              <td>{r.id}</td>
                              <td>{r.name}</td>
                              <td>{r.email}</td>
                              <td>{r.created_at}</td>
                              <td>
                                <Link<Route> to={Route::RustaceansAdd}>
                                  {"edit"}
                                </Link<Route>>
                                <span>{" / "}</span>
                                <Link<Route> to={Route::RustaceansAdd}>
                                  {"delete"}
                                </Link<Route>>
                              </td>
                            </tr>
                          }
                      }).collect::<Html>()
                    }
                  </tbody>
                </table>
              </>
            }
        }
        None => {
            html! {
              <Redirect<Route> to={Route::Login} />
            }
        }
    }
}
