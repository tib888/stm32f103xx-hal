//! Time units

use cortex_m::peripheral::DWT;

use rcc::Clocks;

/// Microseconds
#[derive(Clone, Copy)]
pub struct Microseconds(pub u32);

/// Milliseconds
#[derive(Clone, Copy)]
pub struct Milliseconds(pub u32);

/// Seconds
#[derive(Clone, Copy)]
pub struct Seconds(pub u32);

/// Minutes
#[derive(Clone, Copy)]
pub struct Minutes(pub u32);

/// Hours
#[derive(Clone, Copy)]
pub struct Hours(pub u32);

/// Bits per second
#[derive(Clone, Copy)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `Seconds`
    fn s(self) -> Seconds;

    /// Wrap in `Milliseconds`
    fn ms(self) -> Milliseconds;

    /// Wrap in `Microseconds`
    fn us(self) -> Microseconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn s(self) -> Seconds {
        Seconds(self)
    }

    fn ms(self) -> Milliseconds {
        Milliseconds(self)
    }

    fn us(self) -> Microseconds {
        Microseconds(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

impl Into<Microseconds> for Seconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000_000)
    }
}

impl Into<Milliseconds> for Seconds {
    fn into(self) -> Milliseconds {
        Milliseconds(self.0 * 1_000)
    }
}

impl Into<Microseconds> for Milliseconds {
    fn into(self) -> Microseconds {
        Microseconds(self.0 * 1_000)
    }
}

impl Into<Seconds> for Minutes {
    fn into(self) -> Seconds {
        Seconds(self.0 * 60)
    }
}

impl Into<Seconds> for Hours {
    fn into(self) -> Seconds {
        Seconds(self.0 * 3600)
    }
}

impl Into<Minutes> for Hours {
    fn into(self) -> Minutes {
        Minutes(self.0 * 60)
    }
}

/// A monotonic nondecreasing timer
#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: Hertz,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, clocks: Clocks) -> Self {
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or resetted
        drop(dwt);

        MonoTimer {
            frequency: clocks.sysclk(),
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(&self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(&self) -> Instant {
        Instant {
            now: DWT::get_cycle_count(),
        }
    }
}

/// A measurement of a monotonically nondecreasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(&self) -> u32 {
        DWT::get_cycle_count().wrapping_sub(self.now)
    }

    pub fn elapsed_till(&self, till: &Self) -> u32 {
        till.now.wrapping_sub(self.now)
    }

    pub fn elapsed_since(&self, since: &Self) -> u32 {
        self.now.wrapping_sub(since.now)
    }
}
