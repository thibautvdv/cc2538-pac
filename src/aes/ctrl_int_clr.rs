#[doc = "Register `CTRL_INT_CLR` writer"]
pub type W = crate::W<CTRL_INT_CLR_SPEC>;
#[doc = "Field `RESULT_AV` writer - If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
pub type RESULT_AV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` writer - If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
pub type DMA_IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_RD_ERR` writer - If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_RD_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_WR_ERR` writer - If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUS_ERR` writer - If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
pub type DMA_BUS_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
    #[inline(always)]
    #[must_use]
    pub fn result_av(&mut self) -> RESULT_AV_W<CTRL_INT_CLR_SPEC> {
        RESULT_AV_W::new(self, 0)
    }
    #[doc = "Bit 1 - If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<CTRL_INT_CLR_SPEC> {
        DMA_IN_DONE_W::new(self, 1)
    }
    #[doc = "Bit 29 - If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_rd_err(&mut self) -> KEY_ST_RD_ERR_W<CTRL_INT_CLR_SPEC> {
        KEY_ST_RD_ERR_W::new(self, 29)
    }
    #[doc = "Bit 30 - If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_wr_err(&mut self) -> KEY_ST_WR_ERR_W<CTRL_INT_CLR_SPEC> {
        KEY_ST_WR_ERR_W::new(self, 30)
    }
    #[doc = "Bit 31 - If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn dma_bus_err(&mut self) -> DMA_BUS_ERR_W<CTRL_INT_CLR_SPEC> {
        DMA_BUS_ERR_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_INT_CLR_SPEC;
impl crate::RegisterSpec for CTRL_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_clr::W`](W) writer structure"]
impl crate::Writable for CTRL_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_INT_CLR to value 0"]
impl crate::Resettable for CTRL_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
