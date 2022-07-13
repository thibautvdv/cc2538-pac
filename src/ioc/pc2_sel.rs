#[doc = "Register `PC2_SEL` reader"]
pub struct R(crate::R<PC2_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC2_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC2_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC2_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC2_SEL` writer"]
pub struct W(crate::W<PC2_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC2_SEL_SPEC>;
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
impl From<crate::W<PC2_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC2_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC2_sel` reader - Select one peripheral signal output for PC2."]
pub type PC2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC2_sel` writer - Select one peripheral signal output for PC2."]
pub type PC2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PC2_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC2."]
    #[inline(always)]
    pub fn pc2_sel(&self) -> PC2_SEL_R {
        PC2_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC2."]
    #[inline(always)]
    pub fn pc2_sel(&mut self) -> PC2_SEL_W<0> {
        PC2_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc2_sel](index.html) module"]
pub struct PC2_SEL_SPEC;
impl crate::RegisterSpec for PC2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc2_sel::R](R) reader structure"]
impl crate::Readable for PC2_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc2_sel::W](W) writer structure"]
impl crate::Writable for PC2_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC2_SEL to value 0"]
impl crate::Resettable for PC2_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
