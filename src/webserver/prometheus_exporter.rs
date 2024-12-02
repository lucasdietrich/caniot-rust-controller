use crate::{controller::DeviceLabel, shared::SharedHandle, utils::PrometheusExporterTrait};

pub async fn export(shared: &SharedHandle) -> String {
    let mut buf = String::new();

    let stats = shared.controller_handle.get_controller_stats().await;
    let devices_infos = shared.controller_handle.get_devices_infos_list().await;

    buf.push_str(&stats.export(&[]));

    let medium_label = DeviceLabel::Medium("CAN".to_string());
    for device_infos in devices_infos {
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

    buf
}
