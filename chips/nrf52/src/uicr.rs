//! User information configuration registers
//! Minimal implementation to support activation of the reset button on nRF52-DK

use kernel::common::regs::ReadWrite;

const UICR_BASE: usize = 0x10001200;

#[repr(C)]
pub struct UicrRegisters {
    /// Mapping of the nRESET function (see POWER chapter for details)
    /// Address: 0x200 - 0x204
    pub pselreset0: ReadWrite<u32, Pselreset::Register>,
    /// Mapping of the nRESET function (see POWER chapter for details)
    /// Address: 0x204 - 0x208
    pub pselreset1: ReadWrite<u32, Pselreset::Register>,
    /// Access Port protection
    /// Address: 0x208 - 0x20c
    pub approtect: ReadWrite<u32, ApProtect::Register>,
    /// Setting of pins dedicated to NFC functionality: NFC antenna or GPIO
    /// Address: 0x20c - 0x210
    pub nfcpins: ReadWrite<u32, NfcPins::Register>,
}

register_bitfields! [u32,
    /// Task register 
    Pselreset [
        /// GPIO number P0.n onto which Reset is exposed
        PIN OFFSET(0) NUMBITS(5) [],
        /// Connection
        CONNECTION OFFSET(31) NUMBITS(1) [
            DISCONNECTED = 1,
            CONNECTED = 0
        ]
    ],
    /// Access port protection
    ApProtect [
        /// Ready event
        PALL OFFSET(0) NUMBITS(8) [
            /// Enable
            ENABLED = 0x00,
            /// Disable
            DISABLED = 0xff
        ]
    ],
    /// Setting of pins dedicated to NFC functionality: NFC antenna or GPIO
    NfcPins [
        /// Setting pins dedicated to NFC functionality
        PROTECT OFFSET(0) NUMBITS(1) [
            /// Operation as GPIO pins. Same protection as normal GPIO pins
            DISABLED = 0,
            /// Operation as NFC antenna pins. Configures the protection for
            /// NFC operation
            NFC = 1
        ]
    ]
];

pub struct Uicr {
    regs: *const UicrRegisters,
}

impl Uicr {
    pub const fn new() -> Uicr {
        Uicr {
            regs: UICR_BASE as *const UicrRegisters,
        }
    }

    pub fn set_psel0_reset_pin(&self, pin: usize) {
        let regs = unsafe { &*self.regs };
        regs.pselreset0.set(pin as u32);
    }
    pub fn set_psel1_reset_pin(&self, pin: usize) {
        let regs = unsafe { &*self.regs };
        regs.pselreset1.set(pin as u32);
    }
}
