use as_any::AsAny;

use crate::caniot::{self, DeviceId};

use super::Node;

#[async_trait]
pub trait DeviceActionTrait: AsAny + Send {
    // type Result: AsAny + Send;

    // fn get_did(&self) -> DeviceId;
    // fn get_node_controller(&self) -> &Node;
}
