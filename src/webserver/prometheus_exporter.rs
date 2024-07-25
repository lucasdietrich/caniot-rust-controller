use crate::{controller::DeviceLabel, shared::SharedHandle, utils::PrometheusExporterTrait};

pub async fn export(shared: &SharedHandle) -> String {
    let mut buf = String::new();

    let (controller_stats, can_stats) = shared.controller_handle.get_controller_stats().await;
    let devices_infos = shared.controller_handle.get_devices_infos_list().await;

    buf.push_str(&controller_stats.export(&[]));
    buf.push_str(&can_stats.export(&[]));

    for device_infos in devices_infos {
        let device_labels = vec![
            DeviceLabel::Medium("CAN".to_string()),
            DeviceLabel::Mac(format!("{}", device_infos.did.to_u8())),
            DeviceLabel::Class(device_infos.did.class),
            DeviceLabel::SubId(device_infos.did.sub_id),
        ];
        buf.push_str(&device_infos.stats.export(&device_labels));
        buf.push_str(&device_infos.export(&device_labels));
    }

    buf
}
