use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_rustaceans;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(RustaceanList)]
pub fn rustacean_list(props: &Props) -> HtmlResult {
    let rustaceans = use_rustaceans(&props.token)?;

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
                        <Link<Route>
                          to={Route::RustaceansEdit { id: r.id }}
                          classes="link-secondary"
                        >
                          {"edit"}
                        </Link<Route>>
                        <span class="mx-1">{"/"}</span>
                        <Link<Route> to={Route::RustaceansDelete { id: r.id }}
                          classes="link-danger"
                        >
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
