use alloc::{collections::BTreeMap, sync::Arc, vec, vec::Vec};

use spin::Mutex;

use super::BlockDriver;

/// Device manager for registering and accessing block devices
pub struct DeviceManager {
    devices: Mutex<BTreeMap<usize, Arc<dyn BlockDriver>>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            devices: Mutex::new(BTreeMap::new()),
        }
    }

    /// Register a block device with an ID
    pub fn register(&self, id: usize, device: Arc<dyn BlockDriver>) {
        let mut devices = self.devices.lock();
        devices.insert(id, device);
    }

    /// Get a block device by ID
    pub fn get(&self, id: usize) -> Option<Arc<dyn BlockDriver>> {
        let devices = self.devices.lock();
        devices.get(&id).cloned()
    }
}
