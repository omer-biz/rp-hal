//! Portable in-line assembly
//!
//! Replaces `cortex_m::asm` with things that work on RISC-V and Arm.

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod inner {
    #[doc(inline)]
    pub use cortex_m::asm::{delay, dsb, nop, sev, wfe, wfi};

    #[doc(inline)]
    pub use cortex_m::interrupt::{disable as interrupt_disable, enable as interrupt_enable};

    /// Are interrupts currently enabled?
    pub fn interrupts_enabled() -> bool {
        cortex_m::register::primask::read().is_active()
    }

    /// Check if an IRQ is pending
    pub fn interrrupt_is_pending(irq: rp235x_pac::Interrupt) -> bool {
        cortex_m::peripheral::NVIC::is_pending(irq)
    }

    /// Enable an RP235x IRQ
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can break mask-based critical
    /// sections. Do not call inside a critical section.
    pub unsafe fn interrupt_unmask(irq: rp235x_pac::Interrupt) {
        unsafe { cortex_m::peripheral::NVIC::unmask(irq) }
    }

    /// Disable an RP235x IRQ
    pub fn interrupt_mask(irq: rp235x_pac::Interrupt) {
        cortex_m::peripheral::NVIC::mask(irq)
    }

    /// Check if an RP235x IRQ is enabled
    pub fn interrupt_is_enabled(irq: rp235x_pac::Interrupt) -> bool {
        cortex_m::peripheral::NVIC::is_enabled(irq)
    }

    /// Mark an RP235x IRQ as pending
    pub fn interrupt_pend(irq: rp235x_pac::Interrupt) {
        cortex_m::peripheral::NVIC::pend(irq)
    }
}

#[cfg(all(target_arch = "riscv32", target_os = "none"))]
mod inner {
    #[doc(inline)]
    pub use riscv::asm::{delay, nop, wfi};

    #[doc(inline)]
    pub use riscv::interrupt::machine::disable as interrupt_disable;

    #[doc(inline)]
    pub use crate::xh3irq::{
        is_enabled as interrupt_is_enabled, is_pending as interrrupt_is_pending,
        mask as interrupt_mask, pend as interrupt_pend, unmask as interrupt_unmask,
    };

    /// Enable interrupts
    ///
    /// Enable the Machine External interrupt as well as the global interrupt
    /// flag.
    ///
    /// # Safety
    ///
    /// Do not call from inside a critical section.
    #[inline(always)]
    pub unsafe fn interrupt_enable() {
        unsafe {
            riscv::register::mie::set_mext();
            riscv::interrupt::machine::enable();
        }
    }

    /// Send Event
    #[inline(always)]
    pub fn sev() {
        unsafe {
            // This is how h3.unblock is encoded.
            core::arch::asm!("slt x0, x0, x1");
        }
    }

    /// Wait for Event
    pub fn wfe() {
        unsafe {
            // This is how h3.block is encoded.
            core::arch::asm!("slt x0, x0, x0");
        }
    }

    /// Data Synchronization Barrier
    #[inline(always)]
    pub fn dsb() {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe { core::arch::asm!("fence", options(nostack, preserves_flags)) };
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    /// Are interrupts currently enabled?
    #[inline(always)]
    pub fn interrupts_enabled() -> bool {
        riscv::register::mstatus::read().mie()
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    fn MachineExternal() {
        loop {
            let Some(irq) = crate::xh3irq::get_next_interrupt() else {
                return;
            };
            match irq {
                rp235x_pac::Interrupt::TIMER0_IRQ_0 => {
                    extern "Rust" {
                        fn TIMER0_IRQ_0();
                    }
                    unsafe {
                        TIMER0_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::TIMER0_IRQ_1 => {
                    extern "Rust" {
                        fn TIMER0_IRQ_1();
                    }
                    unsafe {
                        TIMER0_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::TIMER0_IRQ_2 => {
                    extern "Rust" {
                        fn TIMER0_IRQ_2();
                    }
                    unsafe {
                        TIMER0_IRQ_2();
                    }
                }
                rp235x_pac::Interrupt::TIMER0_IRQ_3 => {
                    extern "Rust" {
                        fn TIMER0_IRQ_3();
                    }
                    unsafe {
                        TIMER0_IRQ_3();
                    }
                }
                rp235x_pac::Interrupt::TIMER1_IRQ_0 => {
                    extern "Rust" {
                        fn TIMER1_IRQ_0();
                    }
                    unsafe {
                        TIMER1_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::TIMER1_IRQ_1 => {
                    extern "Rust" {
                        fn TIMER1_IRQ_1();
                    }
                    unsafe {
                        TIMER1_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::TIMER1_IRQ_2 => {
                    extern "Rust" {
                        fn TIMER1_IRQ_2();
                    }
                    unsafe {
                        TIMER1_IRQ_2();
                    }
                }
                rp235x_pac::Interrupt::TIMER1_IRQ_3 => {
                    extern "Rust" {
                        fn TIMER1_IRQ_3();
                    }
                    unsafe {
                        TIMER1_IRQ_3();
                    }
                }
                rp235x_pac::Interrupt::PWM_IRQ_WRAP_0 => {
                    extern "Rust" {
                        fn PWM_IRQ_WRAP_0();
                    }
                    unsafe {
                        PWM_IRQ_WRAP_0();
                    }
                }
                rp235x_pac::Interrupt::PWM_IRQ_WRAP_1 => {
                    extern "Rust" {
                        fn PWM_IRQ_WRAP_1();
                    }
                    unsafe {
                        PWM_IRQ_WRAP_1();
                    }
                }
                rp235x_pac::Interrupt::DMA_IRQ_0 => {
                    extern "Rust" {
                        fn DMA_IRQ_0();
                    }
                    unsafe {
                        DMA_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::DMA_IRQ_1 => {
                    extern "Rust" {
                        fn DMA_IRQ_1();
                    }
                    unsafe {
                        DMA_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::DMA_IRQ_2 => {
                    extern "Rust" {
                        fn DMA_IRQ_2();
                    }
                    unsafe {
                        DMA_IRQ_2();
                    }
                }
                rp235x_pac::Interrupt::DMA_IRQ_3 => {
                    extern "Rust" {
                        fn DMA_IRQ_3();
                    }
                    unsafe {
                        DMA_IRQ_3();
                    }
                }
                rp235x_pac::Interrupt::USBCTRL_IRQ => {
                    extern "Rust" {
                        fn USBCTRL_IRQ();
                    }
                    unsafe {
                        USBCTRL_IRQ();
                    }
                }
                rp235x_pac::Interrupt::PIO0_IRQ_0 => {
                    extern "Rust" {
                        fn PIO0_IRQ_0();
                    }
                    unsafe {
                        PIO0_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::PIO0_IRQ_1 => {
                    extern "Rust" {
                        fn PIO0_IRQ_1();
                    }
                    unsafe {
                        PIO0_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::PIO1_IRQ_0 => {
                    extern "Rust" {
                        fn PIO1_IRQ_0();
                    }
                    unsafe {
                        PIO1_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::PIO1_IRQ_1 => {
                    extern "Rust" {
                        fn PIO1_IRQ_1();
                    }
                    unsafe {
                        PIO1_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::PIO2_IRQ_0 => {
                    extern "Rust" {
                        fn PIO2_IRQ_0();
                    }
                    unsafe {
                        PIO2_IRQ_0();
                    }
                }
                rp235x_pac::Interrupt::PIO2_IRQ_1 => {
                    extern "Rust" {
                        fn PIO2_IRQ_1();
                    }
                    unsafe {
                        PIO2_IRQ_1();
                    }
                }
                rp235x_pac::Interrupt::IO_IRQ_BANK0 => {
                    extern "Rust" {
                        fn IO_IRQ_BANK0();
                    }
                    unsafe {
                        IO_IRQ_BANK0();
                    }
                }
                rp235x_pac::Interrupt::IO_IRQ_BANK0_NS => {
                    extern "Rust" {
                        fn IO_IRQ_BANK0_NS();
                    }
                    unsafe {
                        IO_IRQ_BANK0_NS();
                    }
                }
                rp235x_pac::Interrupt::IO_IRQ_QSPI => {
                    extern "Rust" {
                        fn IO_IRQ_QSPI();
                    }
                    unsafe {
                        IO_IRQ_QSPI();
                    }
                }
                rp235x_pac::Interrupt::IO_IRQ_QSPI_NS => {
                    extern "Rust" {
                        fn IO_IRQ_QSPI_NS();
                    }
                    unsafe {
                        IO_IRQ_QSPI_NS();
                    }
                }
                rp235x_pac::Interrupt::SIO_IRQ_FIFO => {
                    extern "Rust" {
                        fn SIO_IRQ_FIFO();
                    }
                    unsafe {
                        SIO_IRQ_FIFO();
                    }
                }
                rp235x_pac::Interrupt::SIO_IRQ_BELL => {
                    extern "Rust" {
                        fn SIO_IRQ_BELL();
                    }
                    unsafe {
                        SIO_IRQ_BELL();
                    }
                }
                rp235x_pac::Interrupt::SIO_IRQ_FIFO_NS => {
                    extern "Rust" {
                        fn SIO_IRQ_FIFO_NS();
                    }
                    unsafe {
                        SIO_IRQ_FIFO_NS();
                    }
                }
                rp235x_pac::Interrupt::SIO_IRQ_BELL_NS => {
                    extern "Rust" {
                        fn SIO_IRQ_BELL_NS();
                    }
                    unsafe {
                        SIO_IRQ_BELL_NS();
                    }
                }
                rp235x_pac::Interrupt::SIO_IRQ_MTIMECMP => {
                    extern "Rust" {
                        fn SIO_IRQ_MTIMECMP();
                    }
                    unsafe {
                        SIO_IRQ_MTIMECMP();
                    }
                }
                rp235x_pac::Interrupt::CLOCKS_IRQ => {
                    extern "Rust" {
                        fn CLOCKS_IRQ();
                    }
                    unsafe {
                        CLOCKS_IRQ();
                    }
                }
                rp235x_pac::Interrupt::SPI0_IRQ => {
                    extern "Rust" {
                        fn SPI0_IRQ();
                    }
                    unsafe {
                        SPI0_IRQ();
                    }
                }
                rp235x_pac::Interrupt::SPI1_IRQ => {
                    extern "Rust" {
                        fn SPI1_IRQ();
                    }
                    unsafe {
                        SPI1_IRQ();
                    }
                }
                rp235x_pac::Interrupt::UART0_IRQ => {
                    extern "Rust" {
                        fn UART0_IRQ();
                    }
                    unsafe {
                        UART0_IRQ();
                    }
                }
                rp235x_pac::Interrupt::UART1_IRQ => {
                    extern "Rust" {
                        fn UART1_IRQ();
                    }
                    unsafe {
                        UART1_IRQ();
                    }
                }
                rp235x_pac::Interrupt::ADC_IRQ_FIFO => {
                    extern "Rust" {
                        fn ADC_IRQ_FIFO();
                    }
                    unsafe {
                        ADC_IRQ_FIFO();
                    }
                }
                rp235x_pac::Interrupt::I2C0_IRQ => {
                    extern "Rust" {
                        fn I2C0_IRQ();
                    }
                    unsafe {
                        I2C0_IRQ();
                    }
                }
                rp235x_pac::Interrupt::I2C1_IRQ => {
                    extern "Rust" {
                        fn I2C1_IRQ();
                    }
                    unsafe {
                        I2C1_IRQ();
                    }
                }
                rp235x_pac::Interrupt::OTP_IRQ => {
                    extern "Rust" {
                        fn OTP_IRQ();
                    }
                    unsafe {
                        OTP_IRQ();
                    }
                }
                rp235x_pac::Interrupt::TRNG_IRQ => {
                    extern "Rust" {
                        fn TRNG_IRQ();
                    }
                    unsafe {
                        TRNG_IRQ();
                    }
                }
                rp235x_pac::Interrupt::PLL_SYS_IRQ => {
                    extern "Rust" {
                        fn PLL_SYS_IRQ();
                    }
                    unsafe {
                        PLL_SYS_IRQ();
                    }
                }
                rp235x_pac::Interrupt::PLL_USB_IRQ => {
                    extern "Rust" {
                        fn PLL_USB_IRQ();
                    }
                    unsafe {
                        PLL_USB_IRQ();
                    }
                }
                rp235x_pac::Interrupt::POWMAN_IRQ_POW => {
                    extern "Rust" {
                        fn POWMAN_IRQ_POW();
                    }
                    unsafe {
                        POWMAN_IRQ_POW();
                    }
                }
                rp235x_pac::Interrupt::POWMAN_IRQ_TIMER => {
                    extern "Rust" {
                        fn POWMAN_IRQ_TIMER();
                    }
                    unsafe {
                        POWMAN_IRQ_TIMER();
                    }
                }
            }
        }
    }

    /// Our default IRQ handler.
    ///
    /// Just panics.
    ///
    /// # Safety
    ///
    /// Do not call this function - it is called automatically when our
    /// `MachineExternal` interrupt handler can't find anything better to call.
    #[no_mangle]
    #[allow(non_snake_case)]
    unsafe fn DefaultIrqHandler() {
        panic!();
    }
}

#[cfg(not(all(any(target_arch = "arm", target_arch = "riscv32"), target_os = "none")))]
mod inner {
    /// Placeholder function to disable interrupts
    pub fn interrupt_disable() {}

    /// Placeholder function to enable interrupts
    ///
    /// # Safety
    ///
    /// Do not call from inside a critical section.
    pub unsafe fn interrupt_enable() {}

    /// Placeholder function to check if interrupts are enabled
    pub fn interrupts_enabled() -> bool {
        false
    }

    /// Placeholder function to wait for an interrupt
    pub fn wfi() {}

    /// Placeholder function to wait for an event
    pub fn wfe() {}

    /// Placeholder function to do nothing
    pub fn nop() {}

    /// Placeholder function to emit a data synchronisation barrier
    pub fn dsb() {}

    /// Placeholder function to wait for some clock cycles
    pub fn delay(_: u32) {}

    /// Placeholder function to emit an event
    pub fn sev() {}

    /// Placeholder function to check if an IRQ is pending
    pub fn interrrupt_is_pending(_irq: rp235x_pac::Interrupt) -> bool {
        false
    }

    /// Placeholder function to enable an IRQ
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can break mask-based critical
    /// sections. Do not call inside a critical section.
    pub unsafe fn interrupt_unmask(_irq: rp235x_pac::Interrupt) {}

    /// Placeholder function to disable an IRQ
    pub fn interrupt_mask(_irq: rp235x_pac::Interrupt) {}

    /// Placeholder function to check if an IRQ is enabled
    pub fn interrupt_is_enabled(_irq: rp235x_pac::Interrupt) -> bool {
        false
    }

    /// Placeholder function to mark an IRQ as pending
    pub fn interrupt_pend(_irq: rp235x_pac::Interrupt) {}
}

#[doc(inline)]
pub use inner::{
    delay, dsb, interrrupt_is_pending, interrupt_disable, interrupt_enable, interrupt_is_enabled,
    interrupt_mask, interrupt_pend, interrupt_unmask, interrupts_enabled, nop, sev, wfe, wfi,
};

/// Run the closure without interrupts
///
/// No critical-section token because we haven't blocked the second core
pub fn interrupt_free<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let active = interrupts_enabled();
    if active {
        interrupt_disable();
    }
    let t = f();
    if active {
        unsafe {
            interrupt_enable();
        }
    }
    t
}

/// Create a static variable which we can grab a mutable reference to exactly once.
#[macro_export]
macro_rules! singleton {
    ($name:ident: $ty:ty = $expr:expr) => {{
        static mut $name: (::core::mem::MaybeUninit<$ty>, ::core::sync::atomic::AtomicBool) =
            (::core::mem::MaybeUninit::uninit(), ::core::sync::atomic::AtomicBool::new(false));

        #[allow(unsafe_code)]
        if unsafe { $name.1.compare_exchange(false, true, ::core::sync::atomic::Ordering::SeqCst, ::core::sync::atomic::Ordering::SeqCst).is_ok() } {
            // If we get here, the bool was false and we were the ones who set it to true.
            // So we have exclusive access.
            let expr = $expr;
            #[allow(unsafe_code)]
            unsafe {
                $name.0 = ::core::mem::MaybeUninit::new(expr);
                Some(&mut *$name.0.as_mut_ptr())
            }
        } else {
            None
        }
    }};
    (: $ty:ty = $expr:expr) => {
        $crate::singleton!(VAR: $ty = $expr)
    };
}
