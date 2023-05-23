macro_rules! routes {
    (use { $($prelude:item)* }
     $($device_name:ident {
        $(async fn $action_name:ident(&$state_var:ident, &$client_var:ident $(,$param_name:ident : $param_type:ty)*) -> $ret_type:ty $fn_inner:block)+
    })+) => {
        use axum::routing::{get, Router};
        use super::SharedState;
        mod prelude {
            $( $prelude )*
        }

        pub fn make_router() -> Router<SharedState> {
            Router::new()
                $(
                    $( .route({
                        let uri = concat!("/", ::paste::paste! { stringify!([<$device_name:lower>]) }, "/", stringify!($action_name));
                        println!("> Setting up action URI: {uri}");
                        uri
                    }, get(self::$device_name::$action_name)) )+
                )+
        }

        $( #[allow(non_snake_case)]
           mod $device_name {
            use paste::paste;
            use serde::Deserialize;
            use axum::{
                extract::{Query, State},
                headers::{authorization::Bearer, Authorization},
                http::StatusCode,
                TypedHeader
            };
            use crate::{
                server::{ApiResult, ApiError, SharedState},
                devices::TapoDeviceInner
            };

            #[allow(unused_imports)]
            use super::prelude::*;

            $(
                paste! {
                    #[derive(Deserialize)]
                    pub struct [<$action_name:camel Params>] {
                        device: String,
                        $( $param_name: $param_type ),*
                    }
                }

                pub(super) async fn $action_name(
                    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
                    Query(query): Query<paste! { [<$action_name:camel Params>] }>,
                    State(state): State<SharedState>
                ) -> ApiResult<$ret_type> {
                    paste! { let [<$action_name:camel Params>] { device $(, $param_name)* } = query; };

                    let state = state.read().await;

                    let session_id = auth_header.0.token();

                    let _ = state
                        .sessions
                        .get(session_id)
                        .ok_or(ApiError::new(StatusCode::FORBIDDEN, "Invalid bearer token"))?;

                    // TODO: session expiration, etc.?

                    let device = state.devices.get(&device).ok_or(ApiError::new(
                        StatusCode::NOT_FOUND,
                        "Provided device name was not found",
                    ))?;

                    let client = match &device.inner {
                        TapoDeviceInner::$device_name(client) => client,
                        _ => return Err(ApiError::new(StatusCode::BAD_REQUEST, format!("This route is reserved to '{}' devices", stringify!($device_name))))
                    };

                    #[allow(unused_variables)]
                    let $state_var = &state;

                    let $client_var = client;

                    $fn_inner
                }
            )+
        }) +
    };
}

routes! {
    use {}

    L530 {
        async fn on(&state, &client) -> () {
            client.on().await?;
            Ok(())
        }

        async fn off(&state, &client) -> () {
            client.off().await?;
            Ok(())
        }

        async fn set_brightness(&state, &client, level: u8) -> () {
            client.set_brightness(level).await?;
            Ok(())
        }

        async fn set_color(&state, &client, color: tapo::requests::Color) -> () {
            client.set_color(color).await?;
            Ok(())
        }
    }
}
