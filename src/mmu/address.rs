use tock_registers::register_bitfields;
use tock_registers::fields::FieldValue;
use super::descriptor::BlockDescriptor;
use core::iter::StepBy;
use core::ops::Range;

register_bitfields! {u64,
    VADescriptor [
        L0 OFFSET(39) NUMBITS(9) [],
        L1 OFFSET(30) NUMBITS(9) [],
        L2 OFFSET(21) NUMBITS(9) [],
        L3 OFFSET(12) NUMBITS(9) [],
        OFFSET OFFSET(0) NUMBITS(12) []
    ]
}

#[derive(Copy, Clone)]
pub enum MMProt {
    NormalReadOnly,
    NormalExecOnly,
    NormalReadWriteAll,
    NormalReadWriteNoExec,
    PrivilegedExecOnly,
    PrivilegedReadOnly,
    PrivilegedReadWrite,
    SecureExecOnly,
    SecureReadOnly,
    SecureReadWrite,
}

#[derive(Copy, Clone)]
pub enum MMType {
    Device,
    Normal,
    NormalNoExec,
    ReadOnly,
    Instruction,
    SystemReserved,
    SystemReadOnly,
    SystemInstruction,
    SecureReadOnly,
    SecureExecOnly,
    SecureReadWrite,
}

pub struct VirtLayout {
    pub indexes: [u64; 4],
    pub offset: u64,
}

pub type MMSegment = (u64, u64);

#[derive(Clone, Copy)]
pub struct MMRegion {
    pub mem: MMSegment,
    pub granule: u64,
}

impl MMRegion {
    pub fn new(segment: MMSegment, granule: u64) -> Self {
        MMRegion {
            mem: (
                segment.0 & (!(granule - 1)),
                (segment.1 + granule - 1) & (!(granule - 1)),
            ),
            granule,
        }
    }

    pub fn inbound(&self, addr: u64) -> bool {
        return self.mem.0 <= addr && addr < self.mem.1;
    }
}

impl MMType {
    pub fn default_prot(&self) -> MMProt {
        match *self {
            Self::Normal => MMProt::NormalReadWriteAll,
            Self::NormalNoExec => MMProt::NormalReadWriteNoExec,
            Self::ReadOnly => MMProt::NormalReadOnly,
            Self::Instruction => MMProt::NormalExecOnly,
            Self::Device => MMProt::PrivilegedReadWrite,
            Self::SystemReserved => MMProt::PrivilegedReadWrite,
            Self::SystemInstruction => MMProt::PrivilegedExecOnly,
            Self::SystemReadOnly => MMProt::PrivilegedReadOnly,
            _ => unimplemented!(),
        }
    }
}

impl From<MMProt> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: MMProt) -> Self {
        match value {
            MMProt::NormalReadWriteAll => {
                BlockDescriptor::UXN_XN::FALSE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RW_ELx_RW_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalReadWriteNoExec => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RW_ELx_RW_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalExecOnly => {
                BlockDescriptor::UXN_XN::FALSE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RO_ELx_RO_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalReadOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RO_ELx_RO_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedReadOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RO_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedExecOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RO_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedReadWrite => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RW_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            _ => unimplemented!(),
        }
    }
}

impl From<MMType> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: MMType) -> Self {
        let prot_fields: FieldValue<u64, BlockDescriptor::Register> = value.default_prot().into();
        let type_fields = match value {
            MMType::Device => BlockDescriptor::SH::CLEAR,
            _ => BlockDescriptor::SH::IS,
        };
        prot_fields
            + type_fields
            + BlockDescriptor::NSE_NG::TRUE
            + BlockDescriptor::VALID::TRUE
            + BlockDescriptor::TYPE::BLOCK
            + BlockDescriptor::AF::TRUE
    }
}

impl IntoIterator for MMRegion {
    type Item = u64;
    type IntoIter = StepBy<Range<u64>>;
    fn into_iter(self) -> Self::IntoIter {
        (self.mem.0..self.mem.1).step_by(self.granule as usize)
    }
}

impl From<u64> for VirtLayout {
    fn from(value: u64) -> Self {
        Self {
            indexes: [
                VADescriptor::L0.read(value),
                VADescriptor::L1.read(value),
                VADescriptor::L2.read(value),
                VADescriptor::L3.read(value),
            ],
            offset: VADescriptor::OFFSET.read(value),
        }
    }
}