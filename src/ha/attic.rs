use std::{sync::Arc, time::Instant};

use crate::{controller::device_filtering::DeviceFilter, shared::Shared};

use super::LOCATION_ATTIC;

pub async fn control_attic_heaters(shared: &Arc<Shared>) {
    let filter = DeviceFilter::ByLocation(LOCATION_ATTIC);

    let instant = Instant::now();

    if let Some(caniot) = shared
        .controller_handle
        .get_caniot_device_infos_by_filter(filter.clone())
        .await
    {
        // debug!("CANIOT {:#?}", caniot);

        if let Some(ble) = shared
            .controller_handle
            .get_copro_devices_by_filter(filter)
            .await
            .get(0)
        {
            // debug!("BLE {:#?}", ble);

            let diff = Instant::now().duration_since(instant);
            println!("Diff: {:?}", diff);
        }
    }
}
