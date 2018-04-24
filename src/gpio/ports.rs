use super::port::{RegisterBlockA, RegisterBlockB, RegisterBlockD};
use super::{AlternateFunction, Error, GpioPort, InputPin, Mode, OutputPin, OutputSpeed,
            OutputType, PinNumber, PortNumber, Resistor};
use stm32f7::stm32f7x6::{gpioa, gpiob, gpiod};

pub struct GpioPorts<'a> {
    port_a: GpioPort<'a, RegisterBlockA<'a>>,
    port_b: GpioPort<'a, RegisterBlockB<'a>>,
    port_c: GpioPort<'a, RegisterBlockD<'a>>,
    port_d: GpioPort<'a, RegisterBlockD<'a>>,
    port_e: GpioPort<'a, RegisterBlockD<'a>>,
    port_f: GpioPort<'a, RegisterBlockD<'a>>,
    port_g: GpioPort<'a, RegisterBlockD<'a>>,
    port_h: GpioPort<'a, RegisterBlockD<'a>>,
    port_i: GpioPort<'a, RegisterBlockD<'a>>,
    port_j: GpioPort<'a, RegisterBlockD<'a>>,
    port_k: GpioPort<'a, RegisterBlockD<'a>>,
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

    pub fn to_input(
        &mut self,
        pin: (PortNumber, PinNumber),
        resistor: Resistor,
    ) -> Result<InputPin, Error> {
        self.port(pin.0).to_input(pin.1, resistor)
    }

    pub fn to_output(
        &mut self,
        pin: (PortNumber, PinNumber),
        out_type: OutputType,
        out_speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<OutputPin, Error> {
        self.port(pin.0)
            .to_output(pin.1, out_type, out_speed, resistor)
    }

    pub fn to_alternate_function(
        &mut self,
        pin: (PortNumber, PinNumber),
        alternate_fn: AlternateFunction,
        typ: OutputType,
        speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<(), Error> {
        self.port(pin.0)
            .to_alternate_function(pin.1, alternate_fn, typ, speed, resistor)
    }

    pub fn to_alternate_function_all(
        &mut self,
        pins: &[(PortNumber, PinNumber)],
        alternate_fn: AlternateFunction,
        typ: OutputType,
        speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<(), Error> {
        // check that all pins are unused
        let mut pin_in_use = [
            self.port_a.pin_in_use,
            self.port_b.pin_in_use,
            self.port_c.pin_in_use,
            self.port_d.pin_in_use,
            self.port_e.pin_in_use,
            self.port_f.pin_in_use,
            self.port_g.pin_in_use,
            self.port_h.pin_in_use,
            self.port_i.pin_in_use,
            self.port_j.pin_in_use,
            self.port_k.pin_in_use,
        ];
        for &(port, pin) in pins {
            if pin_in_use[port as usize][pin as usize] {
                return Err(Error::PinAlreadyInUse(port, pin));
            } else {
                pin_in_use[port as usize][pin as usize] = true;
            }
        }

        // configure the pins for each port
        use self::PortNumber::*;
        let ports = [
            PortA, PortB, PortC, PortD, PortE, PortF, PortG, PortH, PortI, PortJ, PortK,
        ];
        for &port in ports.iter() {
            // create a pin_vec that contains all pins belonging to the port
            let mut pin_vec = ArrayVec::<[_; 16]>::new();
            for pin in pins.iter().filter(|p| p.0 == port).map(|p| p.1) {
                // the array can't be too small since we check for duplicate pins
                assert!(pin_vec.push(pin).is_none());
            }

            // configure the pins as alternate function pins
            self.port(port).to_alternate_function_all(
                pin_vec.as_slice(),
                alternate_fn,
                typ,
                speed,
                resistor,
            )?;
        }
        Ok(())
    }

    pub fn port(&mut self, port: PortNumber) -> &mut GpioPort {
        use self::PortNumber::*;
        match port {
            PortA => &mut self.port_a,
            PortB => &mut self.port_b,
            PortC => &mut self.port_c,
            PortD => &mut self.port_d,
            PortE => &mut self.port_e,
            PortF => &mut self.port_f,
            PortG => &mut self.port_g,
            PortH => &mut self.port_h,
            PortI => &mut self.port_i,
            PortJ => &mut self.port_j,
            PortK => &mut self.port_k,
        }
    }
}
