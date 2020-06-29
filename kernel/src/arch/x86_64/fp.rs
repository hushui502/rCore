// state saved by fxsave64
// 512 bytes
// https://www.felixcloutier.com/x86/fxsave#tbl-3-47
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Default)]
pub struct FpState {
    // 0
    word1: u64,
    word2: u64,
    // 16
    word3: u64,
    mxcsr: u32,
    mxcsr_mask: u32,
    // 32
    mm: [u64; 16],
    // 160
    xmm: [u64; 32],
    // 416
    rest: [u64; 12],
}

impl FpState {
    pub fn new() -> Self {
        assert!(core::mem::size_of::<Self>() == 512);
        Self {
            // default value
            mxcsr: 0x1f80,
            ..Self::default()
        }
    }

    pub fn save(&mut self) {
        unsafe {
            core::arch::x86_64::_fxsave64(self as *mut FpState as *mut u8);
        }
    }

    pub fn restore(&self) {
        unsafe {
            core::arch::x86_64::_fxrstor64(self as *const FpState as *const u8);
        }
    }
}
