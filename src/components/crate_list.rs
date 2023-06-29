use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_crates;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(CrateList)]
pub fn crate_list(props: &Props) -> HtmlResult {
    let crates = use_crates(&props.token)?;

    Ok(html! {
      <>
        <p>
          <Link<Route> to={Route::CratesAdd}>
            {"+ Add new Crate"}
          </Link<Route>>
        </p>
        <table class="table">
          <thead>
            <tr>
              <th>{"ID"}</th>
              <th>{"Code"}</th>
              <th>{"Name"}</th>
              <th>{"Rustacean ID"}</th>
              <th>{"Version"}</th>
              <th>{"Description"}</th>
              <th>{"Created at"}</th>
              <th>{"Operations"}</th>
            </tr>
          </thead>
          <tbody>
            {
              crates.into_iter().map(|c| {
                  html! {
                    <tr>
                      <td>{c.id}</td>
                      <td>{c.code}</td>
                      <td>{c.name}</td>
                      <td>{c.rustacean_id}</td>
                      <td>{c.version}</td>
                      <td>{c.description}</td>
                      <td>{c.created_at}</td>
                      <td>
                        <Link<Route>
                          to={Route::CratesEdit { id: c.id }}
                          classes="link-secondary"
                        >
                          {"edit"}
                        </Link<Route>>
                        <span class="mx-1">{"/"}</span>
                        <Link<Route> to={Route::CratesDelete { id: c.id }}
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
