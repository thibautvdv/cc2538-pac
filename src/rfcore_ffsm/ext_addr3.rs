#[doc = "Register `EXT_ADDR3` reader"]
pub struct R(crate::R<EXT_ADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_ADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_ADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_ADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_ADDR3` writer"]
pub struct W(crate::W<EXT_ADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_ADDR3_SPEC>;
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
impl From<crate::W<EXT_ADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_ADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_ADDR3` reader - EXT_ADDR\\[31:24\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT_ADDR3` writer - EXT_ADDR\\[31:24\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXT_ADDR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[31:24\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr3(&self) -> EXT_ADDR3_R {
        EXT_ADDR3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[31:24\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr3(&mut self) -> EXT_ADDR3_W<0> {
        EXT_ADDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_addr3](index.html) module"]
pub struct EXT_ADDR3_SPEC;
impl crate::RegisterSpec for EXT_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_addr3::R](R) reader structure"]
impl crate::Readable for EXT_ADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_addr3::W](W) writer structure"]
impl crate::Writable for EXT_ADDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_ADDR3 to value 0"]
impl crate::Resettable for EXT_ADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
