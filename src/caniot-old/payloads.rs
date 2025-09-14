use std::fmt::Debug;

use serde::Serialize;

use super::ProtocolError;

// Sealed trait to ensure that only the allowed types are used as payload types
// TODO suppress warning
pub trait PayloadType {
    const MAX_SIZE: usize = 8;

    fn max_size() -> usize {
        Self::MAX_SIZE
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]

// Represents a Payload of type Command
pub enum Cd {}
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]

// Represents a Payload of type Class Command
pub enum ClCd {}

// Represents a Payload of type Telemetry
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Ty {}

impl PayloadType for Cd {}
impl PayloadType for ClCd {
    const MAX_SIZE: usize = 7;
}
impl PayloadType for Ty {}

#[derive(Clone, Serialize, PartialEq, Eq)]
pub struct Payload<K: PayloadType> {
    data: Vec<u8>,
    marker: std::marker::PhantomData<K>,
}

// Custom implementation of the Debug trait for the Payload type
// to avoid printing the the marker field
impl<K> Debug for Payload<K>
where
    K: PayloadType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Payload")
            .field("data", &self.data)
            .field("size", &self.data.len())
            .finish()
    }
}

impl<K> Payload<K>
where
    K: PayloadType,
{
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
        let mut truncated_data = Vec::with_capacity(K::MAX_SIZE);
        truncated_data.extend_from_slice(&data[..K::MAX_SIZE]);
        Self {
            data: truncated_data,
            marker: std::marker::PhantomData,
        }
    }

    pub fn new(data: impl AsRef<[u8]>) -> Result<Self, ProtocolError> {
        let data = data.as_ref();
        if data.len() > K::MAX_SIZE {
            return Err(ProtocolError::BufferSizeError);
        }

        Ok(Self {
            data: data.to_vec(),
            marker: std::marker::PhantomData,
        })
    }

    pub fn new_from_vec(data: Vec<u8>) -> Result<Self, ProtocolError> {
        if data.len() > K::MAX_SIZE {
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

    pub fn max_size() -> usize {
        K::MAX_SIZE
    }
}

pub trait AsPayload<K: PayloadType>:
    for<'s> TryFrom<&'s Payload<K>, Error = ProtocolError> + Into<Payload<K>> + Clone
{
    fn to_payload(&self) -> Payload<K> {
        // Ty to remove the clone here and auto implement Into<Payload<K>> for &T where T: AsPayload
        self.clone().into()
    }

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

impl Payload<Ty> {}
