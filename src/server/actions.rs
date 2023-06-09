macro_rules! routes {
    (use mod { $($prelude:item)* }
     $($device_name:ident $(,$alias_device_name:ident)* {
        $(async fn $action_name:ident(&$state_var:ident, &$client_var:ident $(,$param_name:ident : $param_type:ty)*) -> $ret_type:ty $fn_inner:block)+
    })+) => {
        use axum::routing::{get, Router};
        use serde::{Serialize, Deserialize};
        use super::SharedState;

        mod prelude {
            $( $prelude )*
        }

        #[derive(Serialize, Deserialize)]
        pub enum TapoDeviceType {
            $(
                $device_name,
                $( $alias_device_name, )*
            )+
        }

        pub fn make_router() -> Router<SharedState> {
            let mut router = Router::new();

            $(
                for device_name in [
                    ::paste::paste! { stringify!([<$device_name:lower>]) }
                    $(, ::paste::paste! { stringify!([<$alias_device_name:lower>]) } )*
                ] {
                    $(
                        let uri = format!("/{device_name}/{}", stringify!($action_name).replace("_", "-"));
                        println!("> Setting up action URI: {uri}");

                        router = router.route(&uri, get(self::$device_name::$action_name));
                    )+
                }
            )+

            router
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

            macro_rules! validate_client_type {
                ($client: expr) => {
                    match $client {
                        TapoDeviceInner::$device_name(client) => Some(client),
                        $( TapoDeviceInner::$alias_device_name(client) => Some(client), )*
                        _ => None
                    }
                }
            }

            #[allow(unused_imports)]
            use super::prelude::*;

            static DEVICE_NAME: [&str; 1 $(+ { stringify!($alias_device_name); 1 })*] = [
                stringify!($device_name),
                $( stringify!($alias_device_name) ),*
            ];

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

                    let client = validate_client_type!(&device.inner).ok_or_else(|| {
                        ApiError::new(
                            StatusCode::BAD_REQUEST,
                            format!(
                                "This route is reserved to '{}' devices",
                                DEVICE_NAME
                                    .iter()
                                    .map(|name| format!("'{name}'"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        )
                    })?;

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
    use mod {
        pub use axum::Json;
        pub use tapo::{
            requests::{Color, LightingEffectPreset, EnergyDataInterval},
            responses::{
                L510DeviceInfoResult,
                L530DeviceInfoResult,
                L930DeviceInfoResult,
                PlugDeviceInfoResult,
                DeviceUsageResult,
                EnergyUsageResult,
                EnergyDataResult
            }
        };
        pub use time::{Date, OffsetDateTime};
    }

    L510, L610 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn set_brightness(&state, &client, level: u8) -> () {
            client.set_brightness(level).await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<L510DeviceInfoResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    L530, L630, L900 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn set_brightness(&state, &client, level: u8) -> () {
            client.set_brightness(level).await.map_err(Into::into)
        }

        async fn set_color(&state, &client, color: Color) -> () {
            client.set_color(color).await.map_err(Into::into)
        }

        async fn set_hue_saturation(&state, &client, hue: u16, saturation: u8) -> () {
            client.set_hue_saturation(hue, saturation).await.map_err(Into::into)
        }

        async fn set_color_temperature(&state, &client, color_temperature: u16) -> () {
            client.set_color_temperature(color_temperature).await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<L530DeviceInfoResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    L920, L930 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn set_brightness(&state, &client, level: u8) -> () {
            client.set_brightness(level).await.map_err(Into::into)
        }

        async fn set_color(&state, &client, color: Color) -> () {
            client.set_color(color).await.map_err(Into::into)
        }

        async fn set_hue_saturation(&state, &client, hue: u16, saturation: u8) -> () {
            client.set_hue_saturation(hue, saturation).await.map_err(Into::into)
        }

        async fn set_color_temperature(&state, &client, color_temperature: u16) -> () {
            client.set_color_temperature(color_temperature).await.map_err(Into::into)
        }

        async fn set_lighting_effect(&state, &client, lighting_effect: LightingEffectPreset) -> () {
            client.set_lighting_effect(lighting_effect).await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<L930DeviceInfoResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    P100, P105 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<PlugDeviceInfoResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    P110, P115 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<PlugDeviceInfoResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }

        async fn get_energy_usage(&state, &client) -> Json<EnergyUsageResult> {
            Ok(Json(client.get_energy_usage().await?))
        }

        async fn get_hourly_energy_data(&state, &client, from: OffsetDateTime, to: OffsetDateTime) -> Json<EnergyDataResult> {
            Ok(Json(client.get_energy_data(EnergyDataInterval::Hourly { start_datetime: from, end_datetime: to }).await?))
        }

        async fn get_daily_energy_data(&state, &client, quarter_start_date: Date) -> Json<EnergyDataResult> {
            Ok(Json(client.get_energy_data(EnergyDataInterval::Daily { start_date: quarter_start_date }).await?))
        }

        async fn get_monthly_energy_data(&state, &client, year_start_date: Date) -> Json<EnergyDataResult> {
            Ok(Json(client.get_energy_data(EnergyDataInterval::Monthly { start_date: year_start_date }).await?))
        }
    }
}
