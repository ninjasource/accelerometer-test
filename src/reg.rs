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
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b0100_0100; // 0x44 or 68
