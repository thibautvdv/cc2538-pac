#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Analog control register"]
    pub ivctrl: IVCTRL,
}
#[doc = "IVCTRL (rw) register accessor: Analog control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ivctrl`]
module"]
pub type IVCTRL = crate::Reg<ivctrl::IVCTRL_SPEC>;
#[doc = "Analog control register"]
pub mod ivctrl;
