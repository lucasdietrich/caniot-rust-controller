use serde::Serialize;

use super::{ProtocolError, SysCtrl};

// Sealed trait to ensure that only the allowed types are used as payload types
// TODO suppress warning
trait PayloadType {}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum CommandPL {}
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum ClassCommandPL {}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum TelemetryPL {}

impl PayloadType for CommandPL {}
impl PayloadType for ClassCommandPL {}
impl PayloadType for TelemetryPL {}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Payload<K: PayloadType> {
    data: Vec<u8>,
    marker: std::marker::PhantomData<K>,
}

impl<K> Payload<K>
where
    K: PayloadType,
{
    const MAX_SIZE: usize = 8;

    pub const EMPTY: Self = Self {
        data: Vec::new(),
        marker: std::marker::PhantomData,
    };

    pub fn new_empty() -> Self {
        Self::EMPTY
    }

    pub fn new_unchecked(data: impl AsRef<[u8]>) -> Self {
        Self::new(data).expect("Failed to create payload")
    }

    pub fn new_truncated(data: impl AsRef<[u8]>) -> Self {
        let data = data.as_ref();
        let mut truncated_data = Vec::with_capacity(Self::MAX_SIZE);
        truncated_data.extend_from_slice(&data[..Self::MAX_SIZE]);
        Self {
            data: truncated_data,
            marker: std::marker::PhantomData,
        }
    }

    pub fn new(data: impl AsRef<[u8]>) -> Result<Self, ProtocolError> {
        let data = data.as_ref();
        if data.len() > Self::MAX_SIZE {
            return Err(ProtocolError::BufferSizeError);
        }

        Ok(Self {
            data: data.to_vec(),
            marker: std::marker::PhantomData,
        })
    }

    fn new_from_vec(data: Vec<u8>) -> Result<Self, ProtocolError> {
        if data.len() > Self::MAX_SIZE {
            return Err(ProtocolError::BufferSizeError);
        }

        Ok(Self {
            data,
            marker: std::marker::PhantomData,
        })
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn into_raw_vec(self) -> Vec<u8> {
        self.data
    }
}

pub trait AsPayload<K: PayloadType>:
    for<'s> TryFrom<&'s Payload<K>, Error = ProtocolError> + Into<Payload<K>> + Clone
{
    fn to_payload(&self) -> Payload<K> {
        // Try to remove the clone here and auto implement Into<Payload<K>> for &T where T: AsPayload
        self.clone().into()
    }

    // TODO implement
    // fn as_ref(&self) -> &[u8] {
    //     self.to_payload().as_ref()
    // }

    fn to_raw_vec(&self) -> Vec<u8> {
        self.to_payload().into_raw_vec()
    }

    fn try_from_raw(data: &[u8]) -> Result<Self, ProtocolError> {
        Payload::<K>::try_from(data).and_then(|payload| Self::try_from(&payload))
    }
}

// Implement the AsPayload trait for all types that implement TryFrom and Into trait with the Payload type
impl<K, T> AsPayload<K> for T
where
    T: for<'s> TryFrom<&'s Payload<K>, Error = ProtocolError> + Into<Payload<K>> + Clone,
    K: PayloadType,
{
}

impl<K> TryFrom<&[u8]> for Payload<K>
where
    K: PayloadType,
{
    type Error = ProtocolError;

    // Implement the AsPayload trait for all types that implement the From trait
    //
    // # Example
    // ```rust
    // fn main() {
    //     let buf = &[0, 1, 2, 3];
    //     let payload = Payload::<TelemetryPL>::try_from(buf).unwrap();
    // }
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Payload::<K>::new(value)
    }
}

impl<K> TryFrom<Vec<u8>> for Payload<K>
where
    K: PayloadType,
{
    type Error = ProtocolError;

    // Implement the AsPayload trait for all types that implement the From trait
    //
    // # Example
    // ```rust
    // fn main() {
    //     let vec = vec![0, 1, 2, 3];
    //     let payload = Payload::<TelemetryPL>::try_from(vec).unwrap();
    // }
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Payload::<K>::new_from_vec(value)
    }
}

impl<K> Into<Vec<u8>> for Payload<K>
where
    K: PayloadType,
{
    fn into(self) -> Vec<u8> {
        self.into_raw_vec()
    }
}

impl<K> AsRef<[u8]> for Payload<K>
where
    K: PayloadType,
{
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Payload<TelemetryPL> {}

impl Payload<ClassCommandPL> {
    pub fn get_system_control(&self) -> Result<SysCtrl, ProtocolError> {
        if self.len() >= 8 {
            Ok(SysCtrl::from(self.data[0]))
        } else {
            Err(ProtocolError::ClassCommandSizeError)
        }
    }

    pub fn get_class_payload(&self) -> &[u8] {
        &self.data[..7]
    }

    pub fn from(class_payload: &[u8], sys_ctrl: Option<SysCtrl>) -> Result<Self, ProtocolError> {
        let pl_len = class_payload.len();
        if pl_len > 7 {
            return Err(ProtocolError::ClassCommandSizeError);
        }

        let sys: u8 = sys_ctrl.unwrap_or_default().into();

        let mut data = Vec::with_capacity(8);
        data.extend_from_slice(&class_payload);
        data.extend_from_slice(&[0_u8; 7][..7 - pl_len]);
        data.extend_from_slice(&[sys]);

        Ok(Self {
            data,
            marker: std::marker::PhantomData,
        })
    }
}

impl From<Payload<ClassCommandPL>> for Payload<CommandPL> {
    fn from(value: Payload<ClassCommandPL>) -> Self {
        Self {
            data: value.data,
            marker: std::marker::PhantomData,
        }
    }
}
