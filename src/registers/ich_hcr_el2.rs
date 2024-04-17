use tock_registers::register_bitfields;
use tock_registers::interfaces::*;

register_bitfields! {u64,
    pub ICH_HCR_EL2 [
        EOICOUNT OFFSET(27) NUMBITS(5) [],
        DVIM OFFSET(15) NUMBITS(1) [
            Masked = 0b1,
            Unmasked = 0b0
        ],
        TDIR OFFSET(14) NUMBITS(1) [
            Trapped = 0b1,
            UnTrapped = 0b0
        ],
        TSEI OFFSET(13) NUMBITS(1) [
            Trapped = 0b1,
            UnTrapped = 0b0
        ],
        TALL1 OFFSET(12) NUMBITS(1) [
            Trapped = 0b1,
            UnTrapped = 0b0
        ],
        TALL0 OFFSET(11) NUMBITS(1) [
            Trapped = 0b1,
            UnTrapped = 0b0
        ],
        TC    OFFSET(10) NUMBITS(1) [
            Trapped = 0b1,
            UnTrapped = 0b0
        ],
        VSGEI OFFSET(8) NUMBITS(1) [
            Discard = 0b1,
            Keep = 0b0
        ],
        vGrp1DIE OFFSET(7) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        vGrp1E1E OFFSET(6) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        vGrp0DIE OFFSET(5) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        vGrp0EIE OFFSET(4) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        NPIE     OFFSET(3) NUMBITS(1) [
            Enable  = 0b1,
            Disable = 0b0
        ],
        LPRENPIE OFFSET(2) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        UIE      OFFSET(1) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        EN       OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ],
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICH_HCR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_HCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICH_HCR_EL2::Register;

    sys_coproc_write_raw!(u64, "ICH_HCR_EL2", "x");
}

pub const ICH_HCR_EL2: Reg = Reg {};
