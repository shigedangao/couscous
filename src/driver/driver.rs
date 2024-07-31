use super::kalosm::Kalosm;
#[cfg(feature = "ollama")]
use super::ollama::Ollama;
use super::{DriverError, DriverOperator};

#[derive(Debug, Clone)]
pub enum SupportedDriver {
    Kalosm,
    #[cfg(feature = "ollama")]
    Ollama,
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
    #[cfg(feature = "ollama")]
    ollama: Ollama,
}

impl Driver {
    pub async fn load_driver(&mut self, s: SupportedDriver) -> Result<(), DriverError> {
        self.supported_driver = s.clone();

        match s {
            SupportedDriver::Kalosm => self.kalosm.set_model().await?,
            #[cfg(feature = "ollama")]
            SupportedDriver::Ollama => self.ollama.set_model().await?,
        }

        Ok(())
    }

    pub fn get_driver(&self) -> Box<dyn DriverOperator> {
        match self.supported_driver {
            SupportedDriver::Kalosm => Box::new(self.kalosm.clone()),
            #[cfg(feature = "ollama")]
            SupportedDriver::Ollama => Box::new(self.ollama.clone()),
        }
    }
}
