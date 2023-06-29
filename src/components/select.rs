use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub options: Vec<(AttrValue, AttrValue)>,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
      <>
        <label for={html_id.clone()}>{props.label.clone()}</label>
        <select
          id={html_id}
          class="form-control"
          name={props.name.clone()}
          onchange={props.onchange.clone()}
        >
          {
              props.options.clone().into_iter().map(|o| {
                  html! {
                      <option value={o.0}>{o.1}</option>
                  }
              }).collect::<Html>()
          }
        </select>
      </>
    }
}
