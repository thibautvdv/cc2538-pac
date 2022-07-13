#[doc = "Register `DIR` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
pub type DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIR` writer - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
pub type DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is output Bits cleared: Corresponding pin is input"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
