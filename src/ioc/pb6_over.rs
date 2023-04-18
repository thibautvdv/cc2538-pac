#[doc = "Register `PB6_OVER` reader"]
pub struct R(crate::R<PB6_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB6_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB6_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB6_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB6_OVER` writer"]
pub struct W(crate::W<PB6_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB6_OVER_SPEC>;
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
impl From<crate::W<PB6_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB6_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB6_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PB6_OVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB6_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PB6_OVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB6_OVER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb6_over(&self) -> PB6_OVER_R {
        PB6_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_over(&mut self) -> PB6_OVER_W<0> {
        PB6_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb6_over](index.html) module"]
pub struct PB6_OVER_SPEC;
impl crate::RegisterSpec for PB6_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb6_over::R](R) reader structure"]
impl crate::Readable for PB6_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb6_over::W](W) writer structure"]
impl crate::Writable for PB6_OVER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB6_OVER to value 0x04"]
impl crate::Resettable for PB6_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
