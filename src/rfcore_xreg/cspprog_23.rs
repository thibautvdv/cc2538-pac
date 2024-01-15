#[doc = "Register `CSPPROG_23` reader"]
pub type R = crate::R<CSPPROG_23_SPEC>;
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub type CSP_INSTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CSP_INSTR_R {
        CSP_INSTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPPROG_23_SPEC;
impl crate::RegisterSpec for CSPPROG_23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspprog_23::R`](R) reader structure"]
impl crate::Readable for CSPPROG_23_SPEC {}
#[doc = "`reset()` method sets CSPPROG_23 to value 0"]
impl crate::Resettable for CSPPROG_23_SPEC {
    const RESET_VALUE: u32 = 0;
}
