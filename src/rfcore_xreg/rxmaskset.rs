#[doc = "Register `RXMASKSET` reader"]
pub type R = crate::R<RXMASKSET_SPEC>;
#[doc = "Register `RXMASKSET` writer"]
pub type W = crate::W<RXMASKSET_SPEC>;
#[doc = "Field `RXENMASKSET` reader - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub type RXENMASKSET_R = crate::FieldReader;
#[doc = "Field `RXENMASKSET` writer - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub type RXENMASKSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    #[must_use]
    pub fn rxenmaskset(&mut self) -> RXENMASKSET_W<RXMASKSET_SPEC> {
        RXENMASKSET_W::new(self, 0)
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
#[doc = "RX enabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaskset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaskset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMASKSET_SPEC;
impl crate::RegisterSpec for RXMASKSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmaskset::R`](R) reader structure"]
impl crate::Readable for RXMASKSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmaskset::W`](W) writer structure"]
impl crate::Writable for RXMASKSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMASKSET to value 0"]
impl crate::Resettable for RXMASKSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
