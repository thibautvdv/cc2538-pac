#[doc = "Register `CSPPROG_5` reader"]
pub struct R(crate::R<CSPPROG_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPPROG_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPPROG_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPPROG_5_SPEC>) -> Self {
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
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspprog_5](index.html) module"]
pub struct CSPPROG_5_SPEC;
impl crate::RegisterSpec for CSPPROG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspprog_5::R](R) reader structure"]
impl crate::Readable for CSPPROG_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPPROG_5 to value 0"]
impl crate::Resettable for CSPPROG_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
