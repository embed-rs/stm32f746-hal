//! Reset and Clock Control

use flash::ACR;
use stm32f7::stm32f7x6::{rcc, PWR, RCC};
use time::{Hertz, MegaHertz};

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc(self)
    }
}

/// Constrained RCC peripheral
pub struct Rcc(RCC);

impl Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub fn ahb1(&mut self) -> AHB1 {
        AHB1 { rcc: &mut self.0 }
    }

    /// Advanced Peripheral Bus 1 (APB1) registers
    pub fn apb1(&mut self) -> APB1 {
        APB1 { rcc: &mut self.0 }
    }

    /// Advanced Peripheral Bus 2 (APB2) registers
    pub fn apb2(&mut self) -> APB2 {
        APB2 { rcc: &mut self.0 }
    }

    /// Clock configuration
    pub fn cfgr(&mut self) -> CFGR {
        CFGR { rcc: &mut self.0 }
    }
}

/// AMBA High-performance Bus (AHB) registers
pub struct AHB1<'a> {
    rcc: &'a mut RCC,
}

impl<'a> AHB1<'a> {
    pub(crate) fn enr(&mut self) -> &rcc::AHB1ENR {
        &self.rcc.ahb1enr
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHB1RSTR {
        &self.rcc.ahb1rstr
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1<'a> {
    rcc: &'a mut RCC,
}

impl<'a> APB1<'a> {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        &self.rcc.apb1enr
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        &self.rcc.apb1rstr
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2<'a> {
    rcc: &'a mut RCC,
}

impl<'a> APB2<'a> {
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        &self.rcc.apb2enr
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        &self.rcc.apb2rstr
    }
}

/// Clock configuration
pub struct CFGR<'a> {
    rcc: &'a mut RCC,
}

impl<'a> CFGR<'a> {
    /// Freezes the clock configuration, making it effective
    pub fn freeze(self, acr: &mut ACR, pwr: &mut PWR) -> Clocks {
        // Enable Power Control clock
        self.rcc.apb1enr.modify(|_, w| w.pwren().enabled());
        self.rcc.apb1enr.read(); // delay

        // Reset HSEON and HSEBYP bits before configuring the HSE
        self.rcc.cr.modify(|_, w| {
            w.hseon().clear_bit();
            w.hsebyp().clear_bit()
        });
        // wait till HSE is disabled
        while self.rcc.cr.read().hserdy().bit_is_set() {}
        // turn HSE on
        self.rcc.cr.modify(|_, w| w.hseon().set_bit());
        // wait till HSE is enabled
        while self.rcc.cr.read().hserdy().bit_is_clear() {}

        // disable main PLL
        self.rcc.cr.write(|r| r.pllon().clear_bit());
        while self.rcc.cr.read().pllrdy().bit_is_set() {}

        // Configure the main PLL clock source, multiplication and division factors.
        // HSE is used as clock source. HSE runs at 25 MHz.
        // PLLM = 25: Division factor for the main PLLs (PLL, PLLI2S and PLLSAI) input clock
        // VCO input frequency = PLL input clock frequency / PLLM with 2 ≤ PLLM ≤ 63
        // => VCO input frequency = 25_000 kHz / 25 = 1_000 kHz = 1 MHz
        // PPLM = 432: Main PLL (PLL) multiplication factor for VCO
        // VCO output frequency = VCO input frequency × PLLN with 50 ≤ PLLN ≤ 432
        // => VCO output frequency 1 Mhz * 432 = 432 MHz
        // PPLQ = 0 =^= division factor 2: Main PLL (PLL) division factor for main system clock
        // PLL output clock frequency = VCO frequency / PLLP with PLLP = 2, 4, 6, or 8
        // => PLL output clock frequency = 432 MHz / 2 = 216 MHz
        self.rcc.pllcfgr.write(|r| {
            r.pllsrc().hse();
            r.pllp().div2();
            unsafe {
                r.pllm().bits(25);
                r.plln().bits(432); // 400 for 200 MHz, 432 for 216 MHz
                r.pllq().bits(9); // 8 for 200 MHz, 9 for 216 MHz
            }
            r
        });

        // enable main PLL
        self.rcc.cr.write(|w| w.pllon().set_bit());
        while self.rcc.cr.read().pllrdy().bit_is_clear() {}

        // enable overdrive
        pwr.cr1.modify(|_, w| w.oden().set_bit());
        while pwr.csr1.read().odrdy().bit_is_clear() {}
        // enable overdrive switching
        pwr.cr1.modify(|_, w| w.odswen().set_bit());
        while pwr.csr1.read().odswrdy().bit_is_clear() {}

        // Program the new number of wait states to the LATENCY bits in the FLASH_ACR register
        acr.acr().modify(|_, w| unsafe { w.latency().bits(5) });
        // Check that the new number of wait states is taken into account to access the Flash
        // memory by reading the FLASH_ACR register
        assert_eq!(acr.acr().read().latency().bits(), 5);

        // HCLK Configuration
        // HPRE = system clock not divided: AHB prescaler
        // => AHB clock frequency = system clock / 1 = 216 MHz / 1 = 216 MHz
        self.rcc.cfgr.modify(|_, w| w.hpre().div1());
        // SYSCLK Configuration
        self.rcc.cfgr.modify(|_, w| w.sw().pll());
        while !self.rcc.cfgr.read().sws().is_pll() {}

        // PCLK1 Configuration
        // PPRE1: APB Low-speed prescaler (APB1)
        // => APB low-speed clock frequency = AHB clock / 4 = 216 Mhz / 4 = 54 MHz
        // FIXME: Frequency should not exceed 45 MHz
        self.rcc.cfgr.modify(|_, w| w.ppre1().div4());
        // PCLK2 Configuration
        // PPRE2: APB high-speed prescaler (APB2)
        // => APB high-speed clock frequency = AHB clock / 2 = 216 Mhz / 2 = 108 MHz
        // FIXME: Frequency should not exceed 90 MHz
        self.rcc.cfgr.modify(|_, w| w.ppre2().div2());
        Clocks {
            sysclk: MegaHertz(216).into(),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
