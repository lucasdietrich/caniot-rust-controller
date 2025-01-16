use std::cmp::Ordering;

use crate::caniot;

use super::alert;

pub trait FilterableDevice {
    fn get_filter_name(&self) -> String;

    fn get_filter_location(&self) -> Option<String> {
        None
    }

    fn get_default_order(&self) -> u32 {
        0
    }

    fn get_active_alert(&self) -> Option<alert::DeviceAlert> {
        None
    }

    fn match_criteria(&self, _criteria: &FilterCriteria) -> bool {
        false
    }

    fn has_tag(&self, _tag: &str) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum FilterCriteria {
    CaniotId(caniot::DeviceId),
    CaniotControllerName(&'static str),
    BleMac(&'static str),
}

#[derive(Debug, Default, Clone)]
pub enum DeviceFilter {
    #[default]
    All, // All devices sorted by did
    ByName(&'static str),       // Devices with a specific name
    ByTag(&'static str),        // Devices having a specific tag
    ByLocation(&'static str),   // Devices in a specific location
    ByCriteria(FilterCriteria), // With a specific criteria
    WithActiveAlert, // Devices with active alerts sorted by alert severity (highest first)
}

impl DeviceFilter {
    pub fn get_filter_function<'a, T: FilterableDevice>(&'a self) -> Box<dyn Fn(&T) -> bool + 'a> {
        match self {
            DeviceFilter::All => Box::new(|_| true),
            DeviceFilter::ByName(name) => Box::new(move |device| &device.get_filter_name() == name),
            DeviceFilter::ByTag(tag) => Box::new(move |device| device.has_tag(tag)),
            DeviceFilter::ByLocation(location) => {
                Box::new(move |device| device.get_filter_location().as_deref() == Some(location))
            }
            DeviceFilter::ByCriteria(criteria) => {
                Box::new(move |device| device.match_criteria(criteria))
            }
            DeviceFilter::WithActiveAlert => Box::new(|device| device.get_active_alert().is_some()),
        }
    }

    pub fn get_sort_function<'a, T: FilterableDevice>(
        &'a self,
    ) -> Box<dyn Fn(&T, &T) -> Ordering + 'a> {
        match self {
            DeviceFilter::ByName(_) => {
                Box::new(|a, b| a.get_default_order().cmp(&b.get_default_order()))
            }
            DeviceFilter::All => Box::new(|a, b| a.get_default_order().cmp(&b.get_default_order())),
            DeviceFilter::WithActiveAlert => {
                Box::new(|a, b| alert::cmp_severity(&a.get_active_alert(), &b.get_active_alert()))
            }
            _ => Box::new(|_, _| Ordering::Equal), /* No sorting for some filters */
        }
    }
}
