#[doc = "Register `FSCAL1` reader"]
pub struct R(crate::R<FSCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCAL1` writer"]
pub struct W(crate::W<FSCAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCAL1_SPEC>;
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
impl From<crate::W<FSCAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCO_CURR_CAL_OE` reader - Override current calibration"]
pub struct VCO_CURR_CAL_OE_R(crate::FieldReader<bool, bool>);
impl VCO_CURR_CAL_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_CURR_CAL_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CURR_CAL_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CURR_CAL_OE` writer - Override current calibration"]
pub struct VCO_CURR_CAL_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_CAL_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `VCO_CURR_CAL` reader - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub struct VCO_CURR_CAL_R(crate::FieldReader<u8, u8>);
impl VCO_CURR_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_CURR_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CURR_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CURR_CAL` writer - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub struct VCO_CURR_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `VCO_CURR` reader - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub struct VCO_CURR_R(crate::FieldReader<u8, u8>);
impl VCO_CURR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_CURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CURR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CURR` writer - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub struct VCO_CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&self) -> VCO_CURR_CAL_OE_R {
        VCO_CURR_CAL_OE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&self) -> VCO_CURR_CAL_R {
        VCO_CURR_CAL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&self) -> VCO_CURR_R {
        VCO_CURR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&mut self) -> VCO_CURR_CAL_OE_W {
        VCO_CURR_CAL_OE_W { w: self }
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&mut self) -> VCO_CURR_CAL_W {
        VCO_CURR_CAL_W { w: self }
    }
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&mut self) -> VCO_CURR_W {
        VCO_CURR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscal1](index.html) module"]
pub struct FSCAL1_SPEC;
impl crate::RegisterSpec for FSCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscal1::R](R) reader structure"]
impl crate::Readable for FSCAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscal1::W](W) writer structure"]
impl crate::Writable for FSCAL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCAL1 to value 0"]
impl crate::Resettable for FSCAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
