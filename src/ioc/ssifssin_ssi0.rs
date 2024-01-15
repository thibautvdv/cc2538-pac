#[doc = "Register `SSIFSSIN_SSI0` reader"]
pub type R = crate::R<SSIFSSIN_SSI0_SPEC>;
#[doc = "Register `SSIFSSIN_SSI0` writer"]
pub type W = crate::W<SSIFSSIN_SSI0_SPEC>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type INPUT_SEL_R = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type INPUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<SSIFSSIN_SSI0_SPEC> {
        INPUT_SEL_W::new(self, 0)
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
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifssin_ssi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifssin_ssi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIFSSIN_SSI0_SPEC;
impl crate::RegisterSpec for SSIFSSIN_SSI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifssin_ssi0::R`](R) reader structure"]
impl crate::Readable for SSIFSSIN_SSI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssifssin_ssi0::W`](W) writer structure"]
impl crate::Writable for SSIFSSIN_SSI0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIFSSIN_SSI0 to value 0"]
impl crate::Resettable for SSIFSSIN_SSI0_SPEC {
    const RESET_VALUE: u32 = 0;
}
