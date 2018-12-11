use crate::hal::watchdog::{Watchdog, /*WatchdogDisable,*/ WatchdogEnable};
use crate::time::Microseconds;
use stm32f103xx::IWDG;

/// The independent watchdog is based on a 12-bit downcounter and 8-bit prescaler. It is
/// clocked from an independent 40 kHz internal RC and as it operates independently of the
/// main clock, it can operate in Stop and Standby modes. It can be used either as a watchdog
/// to reset the device when a problem occurs, or as a free-running timer for application timeout
/// management. It is hardware- or software-configurable through the option bytes. The counter
/// can be frozen in debug mode.
pub struct IndependentWatchdog {
    iwdg: IWDG,
}

impl IndependentWatchdog {
    pub fn new(iwdg: IWDG) -> IndependentWatchdog {
        IndependentWatchdog { iwdg: iwdg }
    }

    /// period in microseconds
    fn init(&mut self, period: u32) {
        // When the independent watchdog is started by writing the value 0xCCCC in the Key register
        // (IWDG_KR), the counter starts counting down from the reset value of 0xFFF. When it
        // reaches the end of count value (0x000) a reset signal is generated (IWDG reset)

        //Write access to the IWDG_PR and IWDG_RLR registers is protected. To modify them, first
        //write the code 0x5555 in the IWDG_KR register

        // approx 40khz clock, 4..256 prescaler, 1..0x1000 reload, period in microsec
        // period = (1<<(prescale+2))*(reload+1)*25us
        // max = 256*4096*25us = 26.214400s
        // min = 4*1*25us = 100us
        let pr = 17u32.saturating_sub((period / 25).leading_zeros());
        let reload = (period >> pr) / 100u32;
        let rlr = if reload > 4096 { 0xFFF } else { reload - 1 };

        // Enable write access to IWDG_PR and IWDG_RLR registers.
        self.iwdg.kr.write(|w| unsafe { w.bits(0x5555) }); // enable write to PR, RLR

        // Configure the IWDG prescaler, counter reload value.
        // This reload value will be loaded in the IWDG counter each time the counter
        // is reloaded, then the IWDG will start counting down from this value.
        self.iwdg.pr.write(|w| unsafe { w.bits(pr) }); // Init prescaler
        self.iwdg.rlr.write(|w| unsafe { w.bits(rlr) }); // Init reload
    }

    /// if several reload values or prescaler values are used by application, it is mandatory to wait
    /// until RVU bit is reset before changing the reload value and to wait until PVU bit is reset
    /// before changing the prescaler value. However, after updating the prescaler and/or the
    /// reload value it is not necessary to wait until RVU or PVU is reset before continuing code
    /// execution (even in case of low-power mode entry, the write operation is taken into account
    /// and will complete)
    fn pending_update(&self) -> bool {
        self.iwdg.sr.read().rvu().bit_is_set() || self.iwdg.sr.read().pvu().bit_is_set()
    }

    fn enable(&mut self) {
        // Reload IWDG counter with value defined in the IWDG_RLR register.
        // Start the IWDG, when the IWDG is used in software mode
        // (no need to enable the LSI, it will be enabled by hardware).
        self.iwdg.kr.write(|w| unsafe { w.bits(0xCCCC) }); // Enable (start) the watchdog
    }

    fn reload(&mut self) {
        self.iwdg.kr.write(|w| unsafe { w.bits(0xAAAA) }); // Reload the watchdog
    }

    // this below is not possible because we can not disable the watchdog once it is enabled
    // pub fn split(self) -> IWDG {
    //     self.disable();
    //     self.iwdg
    // }
}

impl Watchdog for IndependentWatchdog {
    /// Triggers the watchdog. This must be done once the watchdog is started
    /// to prevent the processor being reset.
    fn feed(&mut self) {
        self.reload();
    }
}

impl WatchdogEnable for IndependentWatchdog {
    /// Unit of time used by the watchdog
    type Time = Microseconds;

    /// Starts the watchdog with a given period, typically once this is done
    /// the watchdog needs to be kicked periodically or the processor is reset.
    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.init(period.into().0);
        //while self.pending_update() {}
        self.reload();
        self.enable();
    }
}

// this can not be implemented:
// impl WatchdogDisable for IndependentWatchdog {
//     fn disable(&mut self) {
//     }
// }

// TODO:
// The window watchdog is based on a 7-bit downcounter that can be set as free-running. It
// can be used as a watchdog to reset the device when a problem occurs. It is clocked from
// the main clock. It has an early warning interrupt capability and the counter can be frozen in
// debug mode.
// pub struct WindowWatchdog {
//     wwdg: WWDG,
// }
