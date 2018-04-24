//! Flash memory

use stm32f7::stm32f7x6::{flash, FLASH};

/// Extension trait to constrain the FLASH peripheral
pub trait FlashExt {
    /// Constrains the FLASH peripheral to play nicely with the other abstractions
    fn constrain(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts(self)
    }
}

/// Constrained FLASH peripheral
pub struct Parts(FLASH);

impl Parts {
    /// Opaque ACR register
    pub fn acr(&mut self) -> ACR {
        ACR { flash: &mut self.0 }
    }
}

/// Opaque ACR register
pub struct ACR<'a> {
    flash: &'a mut FLASH,
}

impl<'a> ACR<'a> {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        &self.flash.acr
    }
}
