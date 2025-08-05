macro_rules! build_router {
    (use mod { $($prelude:item)* }
     $($device_name:ident $(,$alias_device_name:ident)* {
        $(async fn $action_name:ident(&$state_var:ident, &$client_var:ident $(,$(#[$param_meta:meta])? $param_name:ident : $param_type:ty)*) -> $ret_type:ty $fn_inner:block)+
    })+) => {
        use axum::routing::{get, Router};
        use serde::{Serialize, Deserialize};
        use super::SharedState;

        mod prelude {
            $( $prelude )*
        }

        #[derive(Serialize, Deserialize, Clone, Copy)]
        pub enum TapoDeviceType {
            $(
                $device_name,
                $( $alias_device_name, )*
            )+
        }

        pub fn make_router() -> Router<SharedState> {
            let mut router = Router::new();
            let mut route_uris = vec![];

            $(
                for device_name in [
                    ::paste::paste! { stringify!([<$device_name:lower>]) }
                    $(, ::paste::paste! { stringify!([<$alias_device_name:lower>]) } )*
                ] {
                    $(
                        let uri = format!("/{device_name}/{}", stringify!($action_name).replace("_", "-"));
                        router = router.route(&uri, get(self::$device_name::$action_name));

                        route_uris.push(uri);
                    )+
                }
            )+

            router = router.route("/", get(|| async move {
                route_uris.join("\n")
            }));

            router
        }

        $( #[allow(non_snake_case)]
           mod $device_name {
            use paste::paste;
            use serde::Deserialize;
            use axum::{
                extract::{Query, State},
                http::StatusCode,
            };
            use axum_extra::{
                headers::{authorization::Bearer, Authorization},
                TypedHeader
            };
            use crate::{
                server::{ApiResult, ApiError, SharedState, auth::auth},
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
                        $( $(#[$param_meta])? $param_name: $param_type ),*
                    }
                }

                pub(super) async fn $action_name(
                    auth_header: TypedHeader<Authorization<Bearer>>,
                    Query(query): Query<paste! { [<$action_name:camel Params>] }>,
                    State(state): State<SharedState>
                ) -> ApiResult<$ret_type> {
                    paste! { let [<$action_name:camel Params>] { device $(, $param_name)* } = query; };

                    auth(auth_header, &state.sessions).await?;

                    // TODO: session expiration, etc.?

                    let device = state.devices.get(&device).ok_or(ApiError::new(
                        StatusCode::NOT_FOUND,
                        "Provided device name was not found",
                    ))?;

                    #[allow(unused_variables)]
                    let $state_var = &state;

                    device
                        .with_client(async move |client| {
                            let client = validate_client_type!(client).ok_or_else(|| {
                                ApiError::new(
                                    StatusCode::BAD_REQUEST,
                                    format!(
                                        "This route is reserved to {} devices, but the provided name refers to a {} device",
                                        DEVICE_NAME.join(", "),
                                        client.type_name()
                                    )
                                )
                            })?;

                            let $client_var = client;

                            $fn_inner
                        })
                        .await
                        .map_err(ApiError::from)?
                }
            )+
        }) +
    };
}

build_router! {
    use mod {
        pub use axum::Json;
        pub use tapo::{
            requests::{Color, LightingEffectPreset, EnergyDataInterval},
            responses::{
                CurrentPowerResult,
                DeviceInfoLightResult,
                DeviceInfoColorLightResult,
                DeviceInfoRgbLightStripResult,
                DeviceInfoRgbicLightStripResult,
                DeviceInfoPlugResult,
                DeviceInfoPlugEnergyMonitoringResult,
                DeviceUsageEnergyMonitoringResult,
                DeviceUsageResult,
                EnergyUsageResult,
                EnergyDataResult
            }
        };
        pub use chrono::NaiveDate;
    }

    L510, L520, L610 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn set_brightness(&state, &client, level: u8) -> () {
            client.set_brightness(level).await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<DeviceInfoLightResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageEnergyMonitoringResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    L530, L535, L630 {
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

        async fn get_device_info(&state, &client) -> Json<DeviceInfoColorLightResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageEnergyMonitoringResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    L900 {
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

        async fn get_device_info(&state, &client) -> Json<DeviceInfoRgbLightStripResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageEnergyMonitoringResult> {
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

        async fn get_device_info(&state, &client) -> Json<DeviceInfoRgbicLightStripResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageEnergyMonitoringResult> {
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

        async fn get_device_info(&state, &client) -> Json<DeviceInfoPlugResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageResult> {
            Ok(Json(client.get_device_usage().await?))
        }
    }

    P110, P110M, P115 {
        async fn on(&state, &client) -> () {
            client.on().await.map_err(Into::into)
        }

        async fn off(&state, &client) -> () {
            client.off().await.map_err(Into::into)
        }

        async fn get_device_info(&state, &client) -> Json<DeviceInfoPlugEnergyMonitoringResult> {
            Ok(Json(client.get_device_info().await?))
        }

        async fn get_device_usage(&state, &client) -> Json<DeviceUsageEnergyMonitoringResult> {
            Ok(Json(client.get_device_usage().await?))
        }

        async fn get_energy_usage(&state, &client) -> Json<EnergyUsageResult> {
            Ok(Json(client.get_energy_usage().await?))
        }

        async fn get_hourly_energy_data(&state, &client, start_date: NaiveDate, end_date: Option<NaiveDate>) -> Json<EnergyDataResult> {
            let end_date = end_date.unwrap_or(start_date);

            Ok(Json(client.get_energy_data(EnergyDataInterval::Hourly { start_date, end_date }).await?))
        }

        async fn get_daily_energy_data(&state, &client, start_date: NaiveDate) -> Json<EnergyDataResult> {
            Ok(Json(client.get_energy_data(EnergyDataInterval::Daily { start_date }).await?))
        }

        async fn get_monthly_energy_data(&state, &client, start_date: NaiveDate) -> Json<EnergyDataResult> {
            Ok(Json(client.get_energy_data(EnergyDataInterval::Monthly { start_date }).await?))
        }

        async fn get_current_power(&state, &client) -> Json<CurrentPowerResult> {
            Ok(Json(client.get_current_power().await?))
        }
    }
}
