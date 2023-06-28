use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub alert_type: AlertType,
    pub message: AttrValue,
}

#[derive(PartialEq)]
pub enum AlertType {
    Info,
    _Success,
    Danger,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let alert_type = match props.alert_type {
        AlertType::Info => "info".to_string(),
        AlertType::_Success => "success".to_string(),
        AlertType::Danger => "danger".to_string(),
    };
    html! {
      <div class={format!("alert alert-{}", alert_type)} role="alert">
        {props.message.clone()}
      </div>
    }
}
