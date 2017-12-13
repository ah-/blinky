#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate stm32l1xx;

use core::u16;
use core::fmt::Write;
use core::intrinsics;

use cortex_m::asm;
use cortex_m_semihosting::hio;

use stm32l1xx::{GPIOA, GPIOB, GPIOC, GPIOH, RCC, TIM7};

fn enable_led_gpios() {
    cortex_m::interrupt::free(
        |cs| {
            let gpioa = GPIOA.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            let gpioc = GPIOC.borrow(cs);
            let gpioh = GPIOH.borrow(cs);
            let rcc = RCC.borrow(cs);

            rcc.ahbenr.modify(|_, w| w.gpiopaen().set_bit());
            rcc.ahbenr.modify(|_, w| w.gpiopben().set_bit());
            rcc.ahbenr.modify(|_, w| w.gpiopcen().set_bit());
            rcc.ahbenr.modify(|_, w| w.gpiophen().set_bit());

            gpioa.moder.modify(|_, w| unsafe {
                w.moder0().bits(1);
                w.moder1().bits(1);
                w.moder2().bits(1);
                w.moder3().bits(1);
                w.moder4().bits(1);
                w.moder5().bits(1);
                w.moder6().bits(1);
                w.moder7().bits(1);
                w.moder8().bits(1);
                w.moder9().bits(1);
                w.moder10().bits(1);
                w.moder11().bits(1);
                w.moder12().bits(1);
                w.moder15().bits(1)
            });

            gpiob.moder.modify(|_, w| unsafe {
                w.moder0().bits(1);
                w.moder1().bits(1);
                w.moder3().bits(1);
                w.moder4().bits(1);
                w.moder5().bits(1);
                w.moder6().bits(1);
                w.moder7().bits(1);
                w.moder8().bits(1);
                w.moder9().bits(1);
                w.moder10().bits(1);
                w.moder12().bits(1);
                w.moder13().bits(1);
                w.moder14().bits(1);
                w.moder15().bits(1)
            });

            gpioc.moder.modify(|_, w| unsafe {
                w.moder14().bits(1);
                w.moder15().bits(1)
            });

            gpioh.moder.modify(|_, w| unsafe {
                w.moder0().bits(1)
            });
        }
    )
}

fn enable_led_column(column: u8, on: bool) {
    cortex_m::interrupt::free(
        |cs| {
            let gpioa = GPIOA.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            let gpioc = GPIOC.borrow(cs);

            match column {
                0 => gpioc.odr.write(|w| w.odr15().bit(on)),
                1 => gpioc.odr.write(|w| w.odr14().bit(on)),
                2 => gpiob.odr.write(|w| w.odr3().bit(on)),
                3 => gpioa.odr.write(|w| w.odr15().bit(on)),
                4 => gpioa.odr.write(|w| w.odr12().bit(on)),
                5 => gpioa.odr.write(|w| w.odr11().bit(on)),
                6 => gpioa.odr.write(|w| w.odr10().bit(on)),
                7 => gpioa.odr.write(|w| w.odr9().bit(on)),
                8 => gpioa.odr.write(|w| w.odr8().bit(on)),
                9 => gpiob.odr.write(|w| w.odr15().bit(on)),
                10 => gpioa.odr.write(|w| w.odr7().bit(on)),
                11 => gpioa.odr.write(|w| w.odr6().bit(on)),
                12 => gpioa.odr.write(|w| w.odr5().bit(on)),
                13 => gpioa.odr.write(|w| w.odr4().bit(on)),
                _ => panic!("Invalid led column")
            }
        }
    )
}

fn set_row_color(row: u8, red: bool, green: bool, blue: bool) {
    cortex_m::interrupt::free(
        |cs| {
            let gpioa = GPIOA.borrow(cs);
            let gpiob = GPIOB.borrow(cs);

            match row {
                0 => {
                    gpiob.bsrr.write(|w| w.bs0().bit(red));
                    gpioa.bsrr.write(|w| w.bs0().bit(green));
                    gpiob.bsrr.write(|w| w.bs14().bit(blue));
                },
                1 => {
                    gpiob.bsrr.write(|w| w.bs1().bit(red));
                    gpioa.bsrr.write(|w| w.bs1().bit(green));
                    gpiob.bsrr.write(|w| w.bs4().bit(blue));
                },
                2 => {
                    gpiob.bsrr.write(|w| w.bs12().bit(red));
                    gpioa.bsrr.write(|w| w.bs2().bit(green));
                    gpiob.bsrr.write(|w| w.bs5().bit(blue));
                },
                3 => {
                    gpiob.bsrr.write(|w| w.bs13().bit(red));
                    gpioa.bsrr.write(|w| w.bs3().bit(green));
                    gpiob.bsrr.write(|w| w.bs6().bit(blue));
                },
                4 => {
                    gpiob.bsrr.write(|w| w.bs8().bit(red));
                    gpiob.bsrr.write(|w| w.bs7().bit(green));
                    gpiob.bsrr.write(|w| w.bs9().bit(blue));
                },
                _ => panic!("Invalid led row")
            }
        }
    )
}

fn main() {
    //let mut stdout = hio::hstdout().unwrap();
    enable_led_gpios();
    cortex_m::interrupt::free(
        |cs| {
            let rcc = RCC.borrow(cs);
            rcc.apb1enr.modify(|_, w| w.tim7en().set_bit());

            let tim7 = TIM7.borrow(cs);

            let ratio = 8_000_000 / 8192;
            let psc = ((ratio - 1) / (u16::MAX as u32)) as u16;
            tim7.psc.write(|w| unsafe { w.psc().bits(psc) });
            let arr = (ratio / ((psc + 1) as u32)) as u16;
            tim7.arr.write(|w| unsafe { w.arr().bits(arr) });
            tim7.cr1.write(|w| w.opm().clear_bit());

            // Start the timer
            tim7.cr1.modify(|_, w| w.cen().set_bit());

            let gpioh = GPIOH.borrow(cs);
            gpioh.odr.write(|w| w.odr0().bit(true));
        }
    );

    let mut i : u8 = 0;
    let mut j : u32 = 0;
    let mut ctr : u16 = 0;

    loop {
        enable_led_column(i, false);

        i = (i + 1) % 14;
        enable_led_column(i, true);

        set_row_color(0, ctr % 3 == 0, ctr % 3 == 1, ctr % 3 == 2);
        set_row_color(1, ctr % 3 == 1, ctr % 3 == 2, ctr % 3 == 0);
        set_row_color(2, ctr % 3 == 2, ctr % 3 == 0, ctr % 3 == 1);
        set_row_color(3, ctr % 3 == 0, ctr % 3 == 1, ctr % 3 == 2);
        set_row_color(4, ctr % 3 == 1, ctr % 3 == 2, ctr % 3 == 0);

        j += 1;
        if (j >= 1024) {
            ctr = (ctr + 1) % 32;
            j = 0;
        }

        cortex_m::interrupt::free(|cs| {
            let tim7 = TIM7.borrow(cs);
            while tim7.sr.read().uif().bit_is_clear() {}
            tim7.sr.modify(|_, w| w.uif().clear_bit());
        });
    }
}


#[lang = "panic_fmt"]
#[no_mangle]
pub unsafe extern "C" fn rust_begin_unwind(
    args: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    col: u32,
) -> ! {
    if let Ok(mut stdout) = hio::hstdout() {
        write!(stdout, "panic at '")
            .and_then(|_| {
                stdout
                    .write_fmt(args)
                    .and_then(|_| writeln!(stdout, "', {}:{}:{}", file, line, col))
            })
            .ok();
    }

    intrinsics::abort()
}


#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
