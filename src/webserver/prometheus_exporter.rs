use crate::{
    shared::SharedHandle,
    utils::{DeviceLabel, PrometheusExporterTrait},
};

pub async fn export(shared: &SharedHandle) -> String {
    let mut buf = String::new();

    let caniot_controller_stats = shared.controller_handle.get_controller_stats().await;
    let caniot_devices_infos = shared
        .controller_handle
        .get_caniot_devices_infos_list()
        .await;
    let copro_controller_stats = shared.controller_handle.get_copro_controller_stats().await;
    let ble_devices = shared.controller_handle.get_copro_devices_list().await;

    buf.push_str(&caniot_controller_stats.export(&[]));
    buf.push_str(&copro_controller_stats.export(&[]));

    let medium_label = DeviceLabel::Medium("CAN".to_string());
    for device_infos in caniot_devices_infos {
        let mac_label = DeviceLabel::Mac(format!("{}", device_infos.did.to_u8()));
        let class_label = DeviceLabel::Class(device_infos.did.class);
        let sub_id_label = DeviceLabel::SubId(device_infos.did.sub_id);

        let mut device_labels = vec![&medium_label, &mac_label, &class_label, &sub_id_label];
        let controller_label;

        if let Some(controller_name) = device_infos.controller_name.as_ref() {
            controller_label = DeviceLabel::Controller(controller_name.clone());
            device_labels.push(&controller_label);
        }

        buf.push_str(&device_infos.stats.export(&device_labels));
        buf.push_str(&device_infos.export(&device_labels));
    }

    let medium_label = DeviceLabel::Medium("BLE".to_string());
    for device_infos in ble_devices {
        let mac_label = DeviceLabel::Mac(device_infos.ble_addr.mac_string());
        let name_label = DeviceLabel::Name(device_infos.name.clone());

        let device_labels = vec![&name_label, &medium_label, &mac_label];

        buf.push_str(&device_infos.export(&device_labels));
    }

    buf
}
