#![allow(non_upper_case_globals)]

/// Register mapping
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone)]
pub enum Register {
    OUT_T_L = 0x0D,             // Temp sensor output
    OUT_T_H = 0x0E,             // Temp sensor output
    WHO_AM_I = 0x0F,            // Who am I ID
    CTRL1 = 0x20,               // Control
    CTRL2 = 0x21,               // Control
    CTRL3 = 0x22,               // Control
    CTRL4_INT1_PAD_CTRL = 0x23, // Control
    CTRL5_INT2_PAD_CTRL = 0x24, // Control
    CTRL6 = 0x25,               // Control
    OUT_T = 0x26,               // Temp sensor output
    STATUS = 0x27,              // Status data
    OUT_X_L = 0x28,             // Output
    OUT_X_H = 0x29,             // Output
    OUT_Y_L = 0x2A,             // Output
    OUT_Y_H = 0x2B,             // Output
    OUT_Z_L = 0x2C,             // Output
    OUT_Z_H = 0x2D,             // Output
    FIFO_CTRL = 0x2E,           // FIFO control
    FIFO_SAMPLES = 0x2F,        // Unread samples stored in FIFO
    TAP_THS_X = 0x30,           // Tap thresholds
    TAP_THS_Y = 0x31,           // Tap thresholds
    TAP_THS_Z = 0x32,           // Tap thresholds
    INT_DUR = 0x33,             // Interrupt duration
    WAKE_UP_THS = 0x34,         // Tap/double-tap selection, inactivity enable, wakeup threshold
    WAKE_UP_DUR = 0x35,         // Wakeup duration
    FREE_FALL = 0x36,           // Free-fall configuration
    STATUS_DUP = 0x37,          // Status
    WAKE_UP_SRC = 0x38,         // Wakeup source
    TAP_SRC = 0x39,             // Tap source
    SIXD_SRC = 0x3A,            // 6D source
    ALL_INT_SRC = 0x3B,         // All interrupt source
    X_OFS_USR = 0x3C,           // Offset data for wakeup
    Y_OFS_USR = 0x3D,           // Offset data for wakeup
    Z_OFS_USR = 0x3E,           // Offset data for wakeup
    CTRL7 = 0x3F,               // Control
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(non_camel_case_types)]
pub enum OutputDataRate {
    PowerDown = 0x00,        // power down
    Hp12Hz5_Lp1Hz6 = 0x01,   // High-Performance 12.5Hz / Low-Power 1.6 Hz
    Hp12Hz5_Lp12Hz5 = 0x02,  // High-Performance 12.5 Hz / Low-Power mode 12.5 Hz
    Hp25Hz_Lp25Hz = 0x03,    // High-Performance 25 Hz / Low-Power 25 Hz
    Hp50Hz_Lp50Hz = 0x04,    // High-Performance 50 Hz / Low-Power 50 Hz
    Hp100Hz_Lp100Hz = 0x05,  // High-Performance 100 Hz / Low-Power mode 100 Hz
    Hp200Hz_Lp200Hz = 0x06,  // High-Performance 200 Hz / Low-Power mode 200 Hz
    Hp400Hz_Lp200Hz = 0x07,  // High-Performance 400 Hz / Low-Power mode 200 Hz
    Hp800Hz_Lp200Hz = 0x08,  // High-Performance 800 Hz / Low-Power mode 200 Hz
    Hp1600Hz_Lp200Hz = 0x09, // High-Performance 1600 Hz / Low-Power mode 200 Hz
}

pub enum OperatingMode {
    LowPower = 0x00,        // Low-Power Mode (12/14-bit resolution)
    HighPerformance = 0x01, // High-Performance Mode (14-bit resolution)
    SingleOnDemand = 0x02,  // Single data conversion on demand mode (12/14-bit resolution)
}

pub enum LowPowerMode {
    Mode1 = 0x00, // Low-Power Mode 1 (12-bit resolution)
    Mode2 = 0x01, // Low-Power Mode 2 (14-bit resolution)
    Mode3 = 0x02, // Low-Power Mode 3 (14-bit resolution)
    Mode4 = 0x03, // Low-Power Mode 4 (14-bit resolution)
}

pub enum FullScaleSelection {
    PlusMinus2 = 0x00,
    PlusMinus4 = 0x01,
    PlusMinus8 = 0x02,
    PlusMinus16 = 0x03,
}

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b0100_0100; // 0x44 or 68 (decimal)
