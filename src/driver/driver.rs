use super::kalosm::Kalosm;
use super::{DriverError, DriverOperator};

pub enum SupportedDriver {
    Kalosm,
}

impl Default for SupportedDriver {
    fn default() -> Self {
        Self::Kalosm
    }
}

#[derive(Default)]
pub struct Driver {
    supported_driver: SupportedDriver,
    kalosm: Kalosm,
}

impl Driver {
    pub async fn load_driver(&mut self, s: SupportedDriver) -> Result<(), DriverError> {
        match s {
            SupportedDriver::Kalosm => self.kalosm.set_model().await?,
        }

        Ok(())
    }

    pub fn get_driver(&self) -> Box<dyn DriverOperator> {
        match self.supported_driver {
            SupportedDriver::Kalosm => Box::new(self.kalosm.clone()),
        }
    }
}
