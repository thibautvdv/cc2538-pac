#[doc = "Register `DCGCGPT` reader"]
pub struct R(crate::R<DCGCGPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCGPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCGPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCGPT` writer"]
pub struct W(crate::W<DCGCGPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCGPT_SPEC>;
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
impl From<crate::W<DCGCGPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCGPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPT0` reader - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_R = crate::BitReader<bool>;
#[doc = "Field `GPT0` writer - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
pub type GPT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT1` reader - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_R = crate::BitReader<bool>;
#[doc = "Field `GPT1` writer - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
pub type GPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT2` reader - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_R = crate::BitReader<bool>;
#[doc = "Field `GPT2` writer - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
pub type GPT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCGPT_SPEC, bool, O>;
#[doc = "Field `GPT3` reader - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_R = crate::BitReader<bool>;
#[doc = "Field `GPT3` writer - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
pub type GPT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCGPT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    pub fn gpt0(&self) -> GPT0_R {
        GPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    pub fn gpt1(&self) -> GPT1_R {
        GPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    pub fn gpt2(&self) -> GPT2_R {
        GPT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    pub fn gpt3(&self) -> GPT3_R {
        GPT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for GPT0 is gated. 1: Clock for GPT0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt0(&mut self) -> GPT0_W<0> {
        GPT0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for GPT1 is gated. 1: Clock for GPT1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt1(&mut self) -> GPT1_W<1> {
        GPT1_W::new(self)
    }
    #[doc = "Bit 2 - 0: Clock for GPT2 is gated. 1: Clock for GPT2 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt2(&mut self) -> GPT2_W<2> {
        GPT2_W::new(self)
    }
    #[doc = "Bit 3 - 0: Clock for GPT3 is gated. 1: Clock for GPT3 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn gpt3(&mut self) -> GPT3_W<3> {
        GPT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcgpt](index.html) module"]
pub struct DCGCGPT_SPEC;
impl crate::RegisterSpec for DCGCGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcgpt::R](R) reader structure"]
impl crate::Readable for DCGCGPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcgpt::W](W) writer structure"]
impl crate::Writable for DCGCGPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCGPT to value 0"]
impl crate::Resettable for DCGCGPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
