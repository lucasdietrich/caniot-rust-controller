use std::cmp::Ordering;

use crate::{
    caniot::DeviceId,
    controller::{alert, Device},
};

#[derive(Debug, Default)]
pub enum DeviceFilter {
    #[default]
    All, // All devices sorted by did
    ById(DeviceId),  // A single device
    WithActiveAlert, // Devices with active alerts sorted by alert severity (highest first)
}

impl DeviceFilter {
    pub fn get_filter_function<'a>(&'a self) -> Box<dyn Fn(&Device) -> bool + 'a> {
        match self {
            DeviceFilter::All => Box::new(|_| true),
            DeviceFilter::ById(did) => Box::new(move |device| device.did == *did),
            DeviceFilter::WithActiveAlert => Box::new(|device| device.get_alert().is_some()),
        }
    }

    pub fn get_sort_function<'a>(&'a self) -> Box<dyn Fn(&Device, &Device) -> Ordering + 'a> {
        match self {
            DeviceFilter::All => Box::new(|a, b| a.did.cmp(&b.did)),
            DeviceFilter::ById(_) => Box::new(|_, _| Ordering::Equal),
            DeviceFilter::WithActiveAlert => {
                Box::new(|a, b| alert::cmp_severity(&a.get_alert(), &b.get_alert()))
            }
        }
    }
}
