#[doc = "Register `PD4_OVER` reader"]
pub struct R(crate::R<PD4_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD4_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD4_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD4_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD4_OVER` writer"]
pub struct W(crate::W<PD4_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD4_OVER_SPEC>;
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
impl From<crate::W<PD4_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD4_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD4_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD4_OVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD4_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD4_OVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD4_OVER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd4_over(&self) -> PD4_OVER_R {
        PD4_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd4_over(&mut self) -> PD4_OVER_W<0> {
        PD4_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd4_over](index.html) module"]
pub struct PD4_OVER_SPEC;
impl crate::RegisterSpec for PD4_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd4_over::R](R) reader structure"]
impl crate::Readable for PD4_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd4_over::W](W) writer structure"]
impl crate::Writable for PD4_OVER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD4_OVER to value 0x04"]
impl crate::Resettable for PD4_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
