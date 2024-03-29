use anyhow::{anyhow, Result};

use vulkanalia::vk::{self, InstanceV1_0, KhrSurfaceExtension};
use vulkanalia::Instance;



use crate::SuitabilityError;

use crate::app_data::AppData;



#[derive(Copy, Clone, Debug)]
pub(crate) struct QueueFamilyIndices {
    pub(crate) graphics: u32,
    pub(crate) transfer: u32,
    pub(crate) present: u32,
}
 impl QueueFamilyIndices {
    pub(crate) unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance
            .get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);


        let transfer = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::TRANSFER))
            .map(|i| i as u32); 


        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                data.surface,
            )? {
                present = Some(index as u32);
                break;
            }
        }
        if let (Some(graphics),Some(transfer), Some(present)) = (graphics, transfer, present) {
            Ok(Self { graphics, transfer, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}
