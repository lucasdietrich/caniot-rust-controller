use std::fmt::Debug;

use chrono::{DateTime, Utc};

use serde::Serialize;

#[allow(dead_code)]
pub const CANIOT_DEVICE_FILTER_ID: u32 = 1 << 2; /* bit 2 is 1 for response frames */
#[allow(dead_code)]
pub const CANIOT_DEVICE_FILTER_MASK: u32 = 1 << 2; /* bit 2 is 1 to filter frames by direction */

use super::{DeviceId, Endpoint, Type};

pub trait InnerFrameTrait: Serialize + Clone {
    fn get_type(&self) -> Type;
    fn get_endpoint(&self) -> Option<Endpoint>;
    fn get_key(&self) -> Option<u16>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Frame<T>
where
    T: InnerFrameTrait,
{
    pub device_id: DeviceId,
    pub data: T,

    // Optional timestamp for the frame
    pub timestamp: DateTime<Utc>,
}

impl<T> Frame<T>
where
    T: InnerFrameTrait,
{
    pub fn new(device_id: DeviceId, data: T) -> Self {
        Self {
            device_id,
            data,
            timestamp: Utc::now(),
        }
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn get_type(&self) -> Type {
        self.data.get_type()
    }

    pub fn get_endpoint(&self) -> Option<Endpoint> {
        self.data.get_endpoint()
    }

    pub fn get_key(&self) -> Option<u16> {
        self.data.get_key()
    }

    pub fn is_broadcast(&self) -> bool {
        self.device_id.is_broadcast()
    }
}
