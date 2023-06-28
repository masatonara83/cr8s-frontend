use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::CurrentUserContext;
use crate::hooks::use_rustaceans;
use crate::Route;

#[function_component(RustaceanList)]
pub fn rustacean_list() -> HtmlResult {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context id missing");
    let token = current_user_ctx.token.as_ref().expect("Token not found");

    let rustaceans = use_rustaceans(&token)?;
    Ok(html! {
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
    })
}
