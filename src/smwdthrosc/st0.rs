#[doc = "Register `ST0` reader"]
pub struct R(crate::R<ST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST0` writer"]
pub struct W(crate::W<ST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST0_SPEC>;
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
impl From<crate::W<ST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST0` reader - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub struct ST0_R(crate::FieldReader<u8, u8>);
impl ST0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST0` writer - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub struct ST0_W<'a> {
    w: &'a mut W,
}
impl<'a> ST0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    pub fn st0(&mut self) -> ST0_W {
        ST0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer 0 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st0](index.html) module"]
pub struct ST0_SPEC;
impl crate::RegisterSpec for ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st0::R](R) reader structure"]
impl crate::Readable for ST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st0::W](W) writer structure"]
impl crate::Writable for ST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST0 to value 0"]
impl crate::Resettable for ST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
