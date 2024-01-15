#[doc = "Register `NINEBITAMASK` reader"]
pub type R = crate::R<NINEBITAMASK_SPEC>;
#[doc = "Register `NINEBITAMASK` writer"]
pub type W = crate::W<NINEBITAMASK_SPEC>;
#[doc = "Field `MASK` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RANGE` reader - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
pub type RANGE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<NINEBITAMASK_SPEC> {
        MASK_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART 9-bit self address mask The NINEBITAMASK register is used to enable the address mask for 9-bit mode. The lower address bits are masked to create a range of address to be matched with the received address byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ninebitamask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ninebitamask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NINEBITAMASK_SPEC;
impl crate::RegisterSpec for NINEBITAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ninebitamask::R`](R) reader structure"]
impl crate::Readable for NINEBITAMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ninebitamask::W`](W) writer structure"]
impl crate::Writable for NINEBITAMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NINEBITAMASK to value 0"]
impl crate::Resettable for NINEBITAMASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
