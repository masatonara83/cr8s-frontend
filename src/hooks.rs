use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use crate::api::rustaceans::{api_rustacean_show, api_rustaceans, Rustacean};

#[hook]
pub fn use_rustaceans(token: &str) -> SuspensionResult<Vec<Rustacean>> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_handle = use_state(|| {
        let cloned_token = token.to_owned();
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

#[hook]
pub fn use_rustacean(token: &str, id: i32) -> SuspensionResult<Rustacean> {
    let result_handle = use_state(|| None);
    let result = (*result_handle).clone();

    let suspension_handle = use_state(|| {
        let cloned_token = token.to_owned();
        Suspension::from_future(async move {
            match api_rustacean_show(&cloned_token, id).await {
                Ok(rustacean) => result_handle.set(Some(rustacean)),
                Err(_) => result_handle.set(None),
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
