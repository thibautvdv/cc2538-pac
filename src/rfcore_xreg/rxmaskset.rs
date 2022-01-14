#[doc = "Register `RXMASKSET` reader"]
pub struct R(crate::R<RXMASKSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMASKSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMASKSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMASKSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMASKSET` writer"]
pub struct W(crate::W<RXMASKSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMASKSET_SPEC>;
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
impl From<crate::W<RXMASKSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMASKSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENMASKSET` reader - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub struct RXENMASKSET_R(crate::FieldReader<u8, u8>);
impl RXENMASKSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXENMASKSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENMASKSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENMASKSET` writer - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub struct RXENMASKSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENMASKSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskset(&self) -> RXENMASKSET_R {
        RXENMASKSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskset(&mut self) -> RXENMASKSET_W {
        RXENMASKSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX enabling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaskset](index.html) module"]
pub struct RXMASKSET_SPEC;
impl crate::RegisterSpec for RXMASKSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmaskset::R](R) reader structure"]
impl crate::Readable for RXMASKSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmaskset::W](W) writer structure"]
impl crate::Writable for RXMASKSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXMASKSET to value 0"]
impl crate::Resettable for RXMASKSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
