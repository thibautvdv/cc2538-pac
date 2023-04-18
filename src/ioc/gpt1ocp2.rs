#[doc = "Register `GPT1OCP2` reader"]
pub struct R(crate::R<GPT1OCP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT1OCP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPT1OCP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPT1OCP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT1OCP2` writer"]
pub struct W(crate::W<GPT1OCP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT1OCP2_SPEC>;
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
impl From<crate::W<GPT1OCP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPT1OCP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as GPT1OCP2 1: PA1 selected as GPT1OCP2 ... 31: PD7 selected as GPT1OCP2"]
pub type INPUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as GPT1OCP2 1: PA1 selected as GPT1OCP2 ... 31: PD7 selected as GPT1OCP2"]
pub type INPUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPT1OCP2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT1OCP2 1: PA1 selected as GPT1OCP2 ... 31: PD7 selected as GPT1OCP2"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT1OCP2 1: PA1 selected as GPT1OCP2 ... 31: PD7 selected as GPT1OCP2"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt1ocp2](index.html) module"]
pub struct GPT1OCP2_SPEC;
impl crate::RegisterSpec for GPT1OCP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt1ocp2::R](R) reader structure"]
impl crate::Readable for GPT1OCP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt1ocp2::W](W) writer structure"]
impl crate::Writable for GPT1OCP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPT1OCP2 to value 0"]
impl crate::Resettable for GPT1OCP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
