use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICH_VTR_EL2 [
        PRI   OFFSET(29) NUMBITS(3) [],
        PRE   OFFSET(26) NUMBITS(3) [],
        ID    OFFSET(23) NUMBITS(3) [
            BITS_16 = 0b000,
            BITS_24 = 0b001
        ],
        SEIS  OFFSET(22) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        A3V OFFSET(21) NUMBITS(1) [
            Zero = 0b0,
            NonZero = 0b1
        ],
        NV4 OFFSET(20) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        TDS OFFSET(19) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        DVIM OFFSET(18) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        ListRegs OFFSET(0) NUMBITS(4) [

        ]
    ]
}

struct Reg {}

impl Readable for Reg {
    type T = u64;
    type R = ICH_VTR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_VTR_EL2", "x");
}

pub const ICH_VTR_EL2: Reg = Reg {};
