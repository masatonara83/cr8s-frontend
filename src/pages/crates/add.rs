use yew::prelude::*;

use crate::components::crate_form::CrateForm;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;

#[function_component(CratesAdd)]
pub fn crates_add() -> Html {
    html! {
        <div class="container">
            <div class="row">
              <div class="col-sm-auto">
                <Sidebar />
              </div>
              <div class="col mt-3">
                <Header />
                <CrateForm />
              </div>
            </div>
        </div>
    }
}
