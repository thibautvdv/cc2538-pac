#[doc = "Register `CSPPROG_23` reader"]
pub struct R(crate::R<CSPPROG_23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPPROG_23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPPROG_23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPPROG_23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub type CSP_INSTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CSP_INSTR_R {
        CSP_INSTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspprog_23](index.html) module"]
pub struct CSPPROG_23_SPEC;
impl crate::RegisterSpec for CSPPROG_23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspprog_23::R](R) reader structure"]
impl crate::Readable for CSPPROG_23_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPPROG_23 to value 0"]
impl crate::Resettable for CSPPROG_23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
