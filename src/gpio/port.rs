use super::{AlternateFunction, Error, InputPin, Mode, OutputPin, OutputSpeed, OutputType,
            PinNumber, PortNumber, Resistor};
use stm32f7::stm32f7x6::{gpioa, gpiob, gpiod};

pub struct GpioPort<'a, T> {
    pub(super) pin_in_use: [bool; 16],
    port: PortNumber,
    register_block: T,
}

struct RegisterBlock<'a, I, O, M, P> {
    idr: &'a I,
    odr: &'a O,
    moder: &'a M,
    pupdr: &'a P,
}

pub(super) type RegisterBlockA<'a> =
    RegisterBlock<'a, gpioa::IDR, gpioa::ODR, gpioa::MODER, gpioa::PUPDR>;
pub(super) type RegisterBlockB<'a> =
    RegisterBlock<'a, gpiob::IDR, gpiob::ODR, gpiob::MODER, gpiob::PUPDR>;
pub(super) type RegisterBlockD<'a> =
    RegisterBlock<'a, gpiod::IDR, gpiod::ODR, gpiod::MODER, gpiod::PUPDR>;

impl<'a> GpioPort<'a, RegisterBlockA<'a>> {
    pub fn new_a(port: PortNumber, register_block: &'a mut gpioa::RegisterBlock) -> Self {
        GpioPort {
            port,
            pin_in_use: [false; 16],
            register_block: RegisterBlock {
                idr: &register_block.idr,
                odr: &register_block.odr,
                moder: &register_block.moder,
                pupdr: &register_block.pupdr,
            },
        }
    }
}

trait RegisterBlockTrait<'a> {
    type Idr: 'a;
    type Odr: 'a;

    fn idr(&self) -> &'a Self::Idr;
    fn odr(&self) -> &'a Self::Odr;
    fn set_mode(&mut self, pins: &[PinNumber], mode: Mode);
    fn set_resistor(&mut self, pins: &[PinNumber], resistor: Resistor);
    fn set_out_type(&mut self, pins: &[PinNumber], out_type: OutputType);
    fn set_out_speed(&mut self, pins: &[PinNumber], out_speed: OutputSpeed);
    fn set_alternate_fn(&mut self, pins: &[PinNumber], alternate_fn: AlternateFunction);
}

impl<'a> RegisterBlockTrait<'a> for RegisterBlockA<'a> {
    fn idr(&self) -> &'a Self::Idr {
        self.idr
    }

    fn idr(&self) -> &'a Self::Odr {
        self.odr
    }

    fn set_mode(&mut self, pins: &[PinNumber], mode: Mode) {
        use self::PinNumber::*;
        use stm32f7::stm32f7x6::gpioa::moder::MODER15W;

        let variant = match mode {
            Mode::Input => MODER15W::INPUT,
            Mode::Output => MODER15W::OUTPUT,
            Mode::Alternate => MODER15W::ALTERNATE,
            Mode::Analog => MODER15W::ANALOG,
        };

        self.moder.modify(|_, w| {
            for pin in pins {
                match pin {
                    Pin0 => w.moder0().variant(variant),
                    Pin1 => w.moder1().variant(variant),
                    Pin2 => w.moder2().variant(variant),
                    Pin3 => w.moder3().variant(variant),
                    Pin4 => w.moder4().variant(variant),
                    Pin5 => w.moder5().variant(variant),
                    Pin6 => w.moder6().variant(variant),
                    Pin7 => w.moder7().variant(variant),
                    Pin8 => w.moder8().variant(variant),
                    Pin9 => w.moder9().variant(variant),
                    Pin10 => w.moder10().variant(variant),
                    Pin11 => w.moder11().variant(variant),
                    Pin12 => w.moder12().variant(variant),
                    Pin13 => w.moder13().variant(variant),
                    Pin14 => w.moder14().variant(variant),
                    Pin15 => w.moder15().variant(variant),
                };
            }
            w
        })
    }

    fn set_resistor(&mut self, pin: PinNumber, resistor: Resistor) {
        use self::PinNumber::*;
        use stm32f7::stm32f7x6::gpioa::pupdr::PUPDR15W;

        let variant = match resistor {
            Resistor::NoPull => PUPDR15W::FLOATING,
            Resistor::PullUp => PUPDR15W::PULLUP,
            Resistor::PullDown => PUPDR15W::PULLDOWN,
        };

        self.pupdr.modify(|_, w| match pin {
            Pin0 => w.pupdr0().variant(variant),
        });
    }
}

impl<'a, T: RegisterBlockTrait<'a>> GpioPort<'a, T> {
    pub fn to_input(
        &mut self,
        pin: PinNumber,
        resistor: Resistor,
    ) -> Result<InputPin<'a, T::Idr>, Error> {
        self.use_pin(pin)?;

        self.register_block.set_mode(&[pin], Mode::Input);
        self.register_block.set_resistor(&[pin], resistor);

        let registers = &mut self.register_block;

        Ok(InputPin {
            pin: pin,
            input_data: self.register_block.idr(),
        })
    }

    pub fn to_output(
        &mut self,
        pin: PinNumber,
        out_type: OutputType,
        out_speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<OutputPin<'a, T::Odr, T::Bsrr>, Error> {
        self.use_pin(pin)?;

        self.register_block.set_mode(&[pin], Mode::Output);
        self.register_block.set_out_type(&[pin], out_type);
        self.register_block.set_out_speed(&[pin], out_speed);
        self.register_block.set_resistor(&[pin], resistor);

        Ok(OutputPin {
            pin: pin,
            output_data: self.register_block.odr(),
            bit_set_reset: self.register_block.bit_set_reset.clone(),
        })
    }

    pub fn to_alternate_function(
        &mut self,
        pin: PinNumber,
        alternate_fn: AlternateFunction,
        typ: OutputType,
        speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<(), Error> {
        self.to_alternate_function_all(&[pin], alternate_fn, typ, speed, resistor)
    }

    pub fn to_alternate_function_all(
        &mut self,
        pins: &[PinNumber],
        alternate_fn: AlternateFunction,
        typ: OutputType,
        speed: OutputSpeed,
        resistor: Resistor,
    ) -> Result<(), Error> {
        self.use_pins(pins)?;

        self.register_block.set_mode(pins, Mode::Alternate);
        self.register_block.set_resistor(pins, resistor);
        self.register_block.set_out_type(pins, typ);
        self.register_block.set_out_speed(pins, speed);
        self.register_block.set_alternate_fn(pins, alternate_fn);

        Ok(())
    }

    fn use_pin(&mut self, pin: PinNumber) -> Result<(), Error> {
        if self.pin_in_use[pin as usize] {
            Err(Error::PinAlreadyInUse(self.port, pin))
        } else {
            self.pin_in_use[pin as usize] = true;
            Ok(())
        }
    }

    fn use_pins(&mut self, pins: &[PinNumber]) -> Result<(), Error> {
        // create a copy of the pin_in_use array since we only want to modify it in case of success
        let mut pin_in_use = self.pin_in_use;

        for &pin in pins {
            if pin_in_use[pin as usize] {
                return Err(Error::PinAlreadyInUse(self.port, pin));
            } else {
                pin_in_use[pin as usize] = true;
            }
        }

        // success => write back updated pin_in_use array
        self.pin_in_use = pin_in_use;

        Ok(())
    }
}
