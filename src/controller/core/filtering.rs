use std::cmp::Ordering;

use crate::caniot;

use super::alert;

pub trait FilterableSensor {
    fn get_filter_name(&self) -> String;

    fn get_filter_location(&self) -> Option<String> {
        None
    }

    fn get_default_order(&self) -> u32 {
        0
    }

    fn get_active_alert(&self) -> Option<alert::SensorAlert> {
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
    CaniotControllerName(String),
    BleMac(String),
}

#[derive(Debug, Default, Clone)]
pub enum SensorFilter {
    #[default]
    All, // All sensors sorted by did
    ByName(String),             // Sensors with a specific name
    ByTag(String),              // Sensors having a specific tag
    ByLocation(String),         // Sensors in a specific location
    ByCriteria(FilterCriteria), // With a specific criteria
    WithActiveAlert, // Sensors with active alerts sorted by alert severity (highest first)
}

impl SensorFilter {
    pub fn get_filter_function<'a, T: FilterableSensor>(&'a self) -> Box<dyn Fn(&T) -> bool + 'a> {
        match self {
            SensorFilter::All => Box::new(|_| true),
            SensorFilter::ByName(name) => Box::new(move |sensor| &sensor.get_filter_name() == name),
            SensorFilter::ByTag(tag) => Box::new(move |sensor| sensor.has_tag(tag)),
            SensorFilter::ByLocation(location) => {
                Box::new(move |sensor| sensor.get_filter_location().as_deref() == Some(location))
            }
            SensorFilter::ByCriteria(criteria) => {
                Box::new(move |sensor| sensor.match_criteria(criteria))
            }
            SensorFilter::WithActiveAlert => Box::new(|sensor| sensor.get_active_alert().is_some()),
        }
    }

    pub fn get_sort_function<'a, T: FilterableSensor>(
        &'a self,
    ) -> Box<dyn Fn(&T, &T) -> Ordering + 'a> {
        match self {
            SensorFilter::ByName(_) => {
                Box::new(|a, b| a.get_default_order().cmp(&b.get_default_order()))
            }
            SensorFilter::All => Box::new(|a, b| a.get_default_order().cmp(&b.get_default_order())),
            SensorFilter::WithActiveAlert => {
                Box::new(|a, b| alert::cmp_severity(&a.get_active_alert(), &b.get_active_alert()))
            }
            _ => Box::new(|_, _| Ordering::Equal), /* No sorting for some filters */
        }
    }
}
