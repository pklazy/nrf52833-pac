#[repr(C)]
#[doc = "TXD EasyDMA channel"]
#[doc(alias = "TXD")]
pub struct Txd {
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
    list: List,
}
impl Txd {
    #[doc = "0x00 - TXD Data pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
    #[doc = "0x0c - EasyDMA list type"]
    #[inline(always)]
    pub const fn list(&self) -> &List {
        &self.list
    }
}
#[doc = "PTR (rw) register accessor: TXD Data pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "TXD Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Maximum number of bytes in TXD buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`]
module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Maximum number of bytes in TXD buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Number of bytes transferred in the last TXD transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of bytes transferred in the last TXD transaction"]
pub mod amount;
#[doc = "LIST (rw) register accessor: EasyDMA list type\n\nYou can [`read`](crate::Reg::read) this register and get [`list::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`list::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@list`]
module"]
#[doc(alias = "LIST")]
pub type List = crate::Reg<list::ListSpec>;
#[doc = "EasyDMA list type"]
pub mod list;
