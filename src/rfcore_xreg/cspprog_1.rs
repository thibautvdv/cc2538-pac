#[doc = "Register `CSPPROG_1` reader"]
pub type R = crate::R<Cspprog1Spec>;
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub type CspInstrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CspInstrR {
        CspInstrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cspprog1Spec;
impl crate::RegisterSpec for Cspprog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspprog_1::R`](R) reader structure"]
impl crate::Readable for Cspprog1Spec {}
#[doc = "`reset()` method sets CSPPROG_1 to value 0"]
impl crate::Resettable for Cspprog1Spec {
    const RESET_VALUE: u32 = 0;
}
