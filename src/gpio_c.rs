#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    data: DATA,
    _reserved1: [u8; 0x03fc],
    dir: DIR,
    is: IS,
    ibe: IBE,
    iev: IEV,
    ie: IE,
    ris: RIS,
    mis: MIS,
    ic: IC,
    afsel: AFSEL,
    _reserved10: [u8; 0xfc],
    gpiolock: GPIOLOCK,
    gpiocr: GPIOCR,
    _reserved12: [u8; 0x01d8],
    pmux: PMUX,
    p_edge_ctrl: P_EDGE_CTRL,
    usb_ctrl: USB_CTRL,
    _reserved15: [u8; 0x04],
    pi_ien: PI_IEN,
    _reserved16: [u8; 0x04],
    irq_detect_ack: IRQ_DETECT_ACK,
    usb_irq_ack: USB_IRQ_ACK,
    irq_detect_unmask: IRQ_DETECT_UNMASK,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x400 - The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    #[doc = "0x404 - The IS register is the interrupt sense register."]
    #[inline(always)]
    pub const fn is(&self) -> &IS {
        &self.is
    }
    #[doc = "0x408 - The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
    #[inline(always)]
    pub const fn ibe(&self) -> &IBE {
        &self.ibe
    }
    #[doc = "0x40c - The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
    #[inline(always)]
    pub const fn iev(&self) -> &IEV {
        &self.iev
    }
    #[doc = "0x410 - The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x414 - The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
    #[inline(always)]
    pub const fn ris(&self) -> &RIS {
        &self.ris
    }
    #[doc = "0x418 - The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
    #[inline(always)]
    pub const fn mis(&self) -> &MIS {
        &self.mis
    }
    #[doc = "0x41c - The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn ic(&self) -> &IC {
        &self.ic
    }
    #[doc = "0x420 - The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
    #[inline(always)]
    pub const fn afsel(&self) -> &AFSEL {
        &self.afsel
    }
    #[doc = "0x520 - A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
    #[inline(always)]
    pub const fn gpiolock(&self) -> &GPIOLOCK {
        &self.gpiolock
    }
    #[doc = "0x524 - The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
    #[inline(always)]
    pub const fn gpiocr(&self) -> &GPIOCR {
        &self.gpiocr
    }
    #[doc = "0x700 - The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
    #[inline(always)]
    pub const fn pmux(&self) -> &PMUX {
        &self.pmux
    }
    #[doc = "0x704 - The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
    #[inline(always)]
    pub const fn p_edge_ctrl(&self) -> &P_EDGE_CTRL {
        &self.p_edge_ctrl
    }
    #[doc = "0x708 - This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
    #[inline(always)]
    pub const fn usb_ctrl(&self) -> &USB_CTRL {
        &self.usb_ctrl
    }
    #[doc = "0x710 - The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
    #[inline(always)]
    pub const fn pi_ien(&self) -> &PI_IEN {
        &self.pi_ien
    }
    #[doc = "0x718 - If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
    #[inline(always)]
    pub const fn irq_detect_ack(&self) -> &IRQ_DETECT_ACK {
        &self.irq_detect_ack
    }
    #[doc = "0x71c - Same functionality as IRQ_DETECT_ACK, but for USB"]
    #[inline(always)]
    pub const fn usb_irq_ack(&self) -> &USB_IRQ_ACK {
        &self.usb_irq_ack
    }
    #[doc = "0x720 - Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
    #[inline(always)]
    pub const fn irq_detect_unmask(&self) -> &IRQ_DETECT_UNMASK {
        &self.irq_detect_unmask
    }
}
#[doc = "DATA (rw) register accessor: This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
pub mod data;
#[doc = "DIR (rw) register accessor: The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
pub mod dir;
#[doc = "IS (rw) register accessor: The IS register is the interrupt sense register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is`]
module"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "The IS register is the interrupt sense register."]
pub mod is;
#[doc = "IBE (rw) register accessor: The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibe`]
module"]
pub type IBE = crate::Reg<ibe::IBE_SPEC>;
#[doc = "The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
pub mod ibe;
#[doc = "IEV (rw) register accessor: The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iev`]
module"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
pub mod iev;
#[doc = "IE (rw) register accessor: The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
pub mod ie;
#[doc = "RIS (r) register accessor: The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
pub mod ris;
#[doc = "MIS (r) register accessor: The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
pub mod mis;
#[doc = "IC (w) register accessor: The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`]
module"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
pub mod ic;
#[doc = "AFSEL (rw) register accessor: The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsel`]
module"]
pub type AFSEL = crate::Reg<afsel::AFSEL_SPEC>;
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
pub mod afsel;
#[doc = "GPIOLOCK (rw) register accessor: A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiolock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiolock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiolock`]
module"]
pub type GPIOLOCK = crate::Reg<gpiolock::GPIOLOCK_SPEC>;
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
pub mod gpiolock;
#[doc = "GPIOCR (rw) register accessor: The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiocr`]
module"]
pub type GPIOCR = crate::Reg<gpiocr::GPIOCR_SPEC>;
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
pub mod gpiocr;
#[doc = "PMUX (rw) register accessor: The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmux`]
module"]
pub type PMUX = crate::Reg<pmux::PMUX_SPEC>;
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
pub mod pmux;
#[doc = "P_EDGE_CTRL (rw) register accessor: The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_edge_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_edge_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p_edge_ctrl`]
module"]
pub type P_EDGE_CTRL = crate::Reg<p_edge_ctrl::P_EDGE_CTRL_SPEC>;
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
pub mod p_edge_ctrl;
#[doc = "USB_CTRL (rw) register accessor: This register is used to control which edge of the USB controller input generates a power-up interrupt to the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`]
module"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
pub mod usb_ctrl;
#[doc = "PI_IEN (rw) register accessor: The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_ien`]
module"]
pub type PI_IEN = crate::Reg<pi_ien::PI_IEN_SPEC>;
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
pub mod pi_ien;
#[doc = "IRQ_DETECT_ACK (rw) register accessor: If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_detect_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_detect_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_detect_ack`]
module"]
pub type IRQ_DETECT_ACK = crate::Reg<irq_detect_ack::IRQ_DETECT_ACK_SPEC>;
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
pub mod irq_detect_ack;
#[doc = "USB_IRQ_ACK (rw) register accessor: Same functionality as IRQ_DETECT_ACK, but for USB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_irq_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_irq_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_irq_ack`]
module"]
pub type USB_IRQ_ACK = crate::Reg<usb_irq_ack::USB_IRQ_ACK_SPEC>;
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB"]
pub mod usb_irq_ack;
#[doc = "IRQ_DETECT_UNMASK (rw) register accessor: Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_detect_unmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_detect_unmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_detect_unmask`]
module"]
pub type IRQ_DETECT_UNMASK = crate::Reg<irq_detect_unmask::IRQ_DETECT_UNMASK_SPEC>;
#[doc = "Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
pub mod irq_detect_unmask;
