#[doc = "Register `USBCTRL` reader"]
pub struct R(crate::R<USBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCTRL` writer"]
pub struct W(crate::W<USBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCTRL_SPEC>;
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
impl From<crate::W<USBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_STB` reader - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub struct USB_STB_R(crate::FieldReader<bool, bool>);
impl USB_STB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_STB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_STB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_STB` writer - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub struct USB_STB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_STB_W<'a> {
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
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&self) -> USB_STB_R {
        USB_STB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&mut self) -> USB_STB_W {
        USB_STB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY stand-by control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctrl](index.html) module"]
pub struct USBCTRL_SPEC;
impl crate::RegisterSpec for USBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbctrl::R](R) reader structure"]
impl crate::Readable for USBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](W) writer structure"]
impl crate::Writable for USBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for USBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
