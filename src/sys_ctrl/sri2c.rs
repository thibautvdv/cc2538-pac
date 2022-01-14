#[doc = "Register `SRI2C` reader"]
pub struct R(crate::R<SRI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRI2C` writer"]
pub struct W(crate::W<SRI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRI2C_SPEC>;
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
impl From<crate::W<SRI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0` reader - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub struct I2C0_R(crate::FieldReader<bool, bool>);
impl I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0` writer - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the reset for I2C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sri2c](index.html) module"]
pub struct SRI2C_SPEC;
impl crate::RegisterSpec for SRI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sri2c::R](R) reader structure"]
impl crate::Readable for SRI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sri2c::W](W) writer structure"]
impl crate::Writable for SRI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRI2C to value 0"]
impl crate::Resettable for SRI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
