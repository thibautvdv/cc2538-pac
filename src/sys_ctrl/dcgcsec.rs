#[doc = "Register `DCGCSEC` reader"]
pub type R = crate::R<DCGCSEC_SPEC>;
#[doc = "Register `DCGCSEC` writer"]
pub type W = crate::W<DCGCSEC_SPEC>;
#[doc = "Field `PKA` reader - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PKA_R = crate::BitReader;
#[doc = "Field `PKA` writer - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PKA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PKA_W<DCGCSEC_SPEC> {
        PKA_W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<DCGCSEC_SPEC> {
        AES_W::new(self, 1)
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
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCSEC_SPEC;
impl crate::RegisterSpec for DCGCSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcsec::R`](R) reader structure"]
impl crate::Readable for DCGCSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcsec::W`](W) writer structure"]
impl crate::Writable for DCGCSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGCSEC to value 0"]
impl crate::Resettable for DCGCSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
