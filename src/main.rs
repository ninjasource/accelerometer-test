#![no_std]
#![no_main]

extern crate rtt_target;
use accelerometer::{vector::I16x3, Accelerometer, RawAccelerometer};
use core::{convert::Infallible, fmt::Debug};
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::{digital::v2::OutputPin, spi::Mode, spi::Phase, spi::Polarity};
use lis2dw12::{FullScaleSelection, Lis2dw12, OperatingMode, OutputDataRate};
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    delay::Delay,
    gpio::{
        gpioa::{PA4, PA5, PA6, PA7},
        Alternate, Floating, Input, Output, PushPull,
    },
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

type SpiPhysical = Spi<
    SPI1,
    Spi1NoRemap,
    (
        PA5<Alternate<PushPull>>,
        PA6<Input<Floating>>,
        PA7<Alternate<PushPull>>,
    ),
    u8,
>;

type CsPhysical = PA4<Output<PushPull>>;
type Lis2dw12Physical = Lis2dw12<SpiPhysical, CsPhysical>;
type Lis2dw12Error = lis2dw12::Error<stm32f1xx_hal::spi::Error, Infallible>;
type RawAccelerometerError =
    accelerometer::Error<<Lis2dw12Physical as RawAccelerometer<I16x3>>::Error>;

#[derive(Debug)]
enum MainError {
    Accelerometer(Lis2dw12Error),
}

impl From<Lis2dw12Error> for MainError {
    fn from(err: Lis2dw12Error) -> Self {
        Self::Accelerometer(err)
    }
}

impl From<RawAccelerometerError> for MainError {
    fn from(err: RawAccelerometerError) -> Self {
        MainError::Accelerometer(err.into_cause())
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

    // spi setup
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let spi = Spi::spi1(
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

    cs.set_high().unwrap();

    // wait for things to settle
    delay.delay_ms(5_u16);
    rprintln!("[INF] Done initialising");

    let mut lis2dw12 = Lis2dw12::new(spi, cs);
    run(&mut lis2dw12, &mut delay).unwrap();

    loop {
        asm::nop()
    }
}

fn run(accel: &mut Lis2dw12Physical, delay: &mut Delay) -> Result<(), MainError> {
    accel.check_who_am_i()?;
    accel.set_operating_mode(OperatingMode::HighPerformance)?;
    accel.set_low_noise(true)?;
    accel.set_full_scale_selection(FullScaleSelection::PlusMinus2G)?;
    accel.set_output_data_rate(OutputDataRate::Hp100HzLp100Hz)?; // 100 Hz
    run_loop(accel, delay)?;
    Ok(())
}

fn run_loop(accel: &mut Lis2dw12Physical, delay: &mut Delay) -> Result<(), MainError> {
    loop {
        let raw = accel.accel_raw()?;
        let norm = accel.accel_norm()?;
        let sample_rate = accel.sample_rate()?;
        rprintln!(
            "norm: ({:.2}, {:.2}, {:.2}), sample_rate_hz: {}, raw: {:?}",
            norm.x,
            norm.y,
            norm.z,
            sample_rate,
            raw,
        );
        delay.delay_ms(100_u16);
    }
}
