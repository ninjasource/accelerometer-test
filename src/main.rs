#![no_std]
#![no_main]
#![allow(warnings)]

#[macro_use]
extern crate rtt_target;

use accelerometer_test::{FullScaleSelection, OperatingMode, LIS2DW12};
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::{digital::v2::OutputPin, spi::Mode, spi::Phase, spi::Polarity};
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA5, PA6, PA7},
        gpiob, Alternate, Floating, Input, PushPull,
    },
    i2c::{BlockingI2c, DutyCycle, I2c},
    pac::SPI1,
    prelude::*,
    spi::{Spi, Spi1NoRemap},
    stm32,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {
        asm::bkpt() // halt = exit probe-run
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("[INF] Initializing");

    // general peripheral setup
    let cp: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    /*
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        stm32f1xx_hal::i2c::Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );
    */

    // spi setup
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        2.mhz(), // 2mhz is max for bluepill
        clocks,
        &mut rcc.apb2,
    );

    cs.set_high();

    // wait for things to settle
    delay.delay_ms(5_u16);
    rprintln!("[INF] Done initialising");

    let mut lis2dw12 = LIS2DW12::new(cs).unwrap();

    let who_am_i = lis2dw12.get_device_id(&mut spi).unwrap();
    rprintln!("Who Am I: {}", who_am_i);

    lis2dw12.set_mode(&mut spi, OperatingMode::HighPerformance);
    lis2dw12.set_low_noise(&mut spi, true);
    lis2dw12.set_full_scale_selection(&mut spi, FullScaleSelection::PlusMinus2);
    lis2dw12.set_output_data_rate(
        &mut spi,
        accelerometer_test::OutputDataRate::Hp100Hz_Lp100Hz,
    );
    loop {
        let raw = lis2dw12.get_raw(&mut spi).unwrap();
        rprintln!("raw: {:?}", raw);
        delay.delay_ms(100_u16);
    }
}
