use super::port::{RegisterBlockA, RegisterBlockB, RegisterBlockD};
use super::{GpioPort, PortNumber};
use stm32f7::stm32f7x6::{gpioa, gpiob, gpiod};

pub struct GpioPorts<'a> {
    pub port_a: GpioPort<RegisterBlockA<'a>>,
    pub port_b: GpioPort<RegisterBlockB<'a>>,
    pub port_c: GpioPort<RegisterBlockD<'a>>,
    pub port_d: GpioPort<RegisterBlockD<'a>>,
    pub port_e: GpioPort<RegisterBlockD<'a>>,
    pub port_f: GpioPort<RegisterBlockD<'a>>,
    pub port_g: GpioPort<RegisterBlockD<'a>>,
    pub port_h: GpioPort<RegisterBlockD<'a>>,
    pub port_i: GpioPort<RegisterBlockD<'a>>,
    pub port_j: GpioPort<RegisterBlockD<'a>>,
    pub port_k: GpioPort<RegisterBlockD<'a>>,
}

impl<'a> GpioPorts<'a> {
    pub fn new(
        gpio_a: &'a mut gpioa::RegisterBlock,
        gpio_b: &'a mut gpiob::RegisterBlock,
        gpio_c: &'a mut gpiod::RegisterBlock,
        gpio_d: &'a mut gpiod::RegisterBlock,
        gpio_e: &'a mut gpiod::RegisterBlock,
        gpio_f: &'a mut gpiod::RegisterBlock,
        gpio_g: &'a mut gpiod::RegisterBlock,
        gpio_h: &'a mut gpiod::RegisterBlock,
        gpio_i: &'a mut gpiod::RegisterBlock,
        gpio_j: &'a mut gpiod::RegisterBlock,
        gpio_k: &'a mut gpiod::RegisterBlock,
    ) -> Self {
        Self {
            port_a: GpioPort::new_a(PortNumber::PortA, gpio_a),
            port_b: GpioPort::new_b(PortNumber::PortB, gpio_b),
            port_c: GpioPort::new_d(PortNumber::PortC, gpio_c),
            port_d: GpioPort::new_d(PortNumber::PortD, gpio_d),
            port_e: GpioPort::new_d(PortNumber::PortE, gpio_e),
            port_f: GpioPort::new_d(PortNumber::PortF, gpio_f),
            port_g: GpioPort::new_d(PortNumber::PortG, gpio_g),
            port_h: GpioPort::new_d(PortNumber::PortH, gpio_h),
            port_i: GpioPort::new_d(PortNumber::PortI, gpio_i),
            port_j: GpioPort::new_d(PortNumber::PortJ, gpio_j),
            port_k: GpioPort::new_d(PortNumber::PortK, gpio_k),
        }
    }
}
