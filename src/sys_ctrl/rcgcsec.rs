#[doc = "Register `RCGCSEC` reader"]
pub struct R(crate::R<RCGCSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCSEC` writer"]
pub struct W(crate::W<RCGCSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RCGCSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKA` reader - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PKA_R = crate::BitReader<bool>;
#[doc = "Field `PKA` writer - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCSEC_SPEC, bool, O>;
#[doc = "Field `AES` reader - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCSEC_SPEC, bool, O>;
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
    pub fn pka(&mut self) -> PKA_W<0> {
        PKA_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<1> {
        AES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcsec](index.html) module"]
pub struct RCGCSEC_SPEC;
impl crate::RegisterSpec for RCGCSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcsec::R](R) reader structure"]
impl crate::Readable for RCGCSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcsec::W](W) writer structure"]
impl crate::Writable for RCGCSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCSEC to value 0"]
impl crate::Resettable for RCGCSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
