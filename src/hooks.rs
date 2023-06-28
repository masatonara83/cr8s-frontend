use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use crate::api::rustaceans::{api_rustaceans, Rustacean};

#[hook]
pub fn use_rustaceans(token: &String) -> SuspensionResult<Vec<Rustacean>> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_handle = use_state(|| {
        let cloned_token = token.clone();
        Suspension::from_future(async move {
            match api_rustaceans(&cloned_token).await {
                Ok(rustaceans) => result_handle.set(Some(rustaceans)),
                Err(_) => result_handle.set(Some(vec![])),
            }
        })
    });

    let suspension = (*suspension_handle).clone();
    if suspension.resumed() {
        return match result {
            Some(v) => Ok(v),
            None => Err(suspension),
        };
    }

    Err(suspension)
}
