#[doc = "Register `OBSSEL3` reader"]
pub type R = crate::R<OBSSEL3_SPEC>;
#[doc = "Register `OBSSEL3` writer"]
pub type W = crate::W<OBSSEL3_SPEC>;
#[doc = "Field `SEL` reader - n - obs_sigs\\[n\\]
output on output 3: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - n - obs_sigs\\[n\\]
output on output 3: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Observation output 3 enable control for PC3 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC3."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Observation output 3 enable control for PC3 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC3."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 3: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Observation output 3 enable control for PC3 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC3."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - n - obs_sigs\\[n\\]
output on output 3: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<OBSSEL3_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Observation output 3 enable control for PC3 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC3."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<OBSSEL3_SPEC> {
        EN_W::new(self, 7)
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
#[doc = "Select output signal on observation output 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obssel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obssel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBSSEL3_SPEC;
impl crate::RegisterSpec for OBSSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obssel3::R`](R) reader structure"]
impl crate::Readable for OBSSEL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`obssel3::W`](W) writer structure"]
impl crate::Writable for OBSSEL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBSSEL3 to value 0"]
impl crate::Resettable for OBSSEL3_SPEC {
    const RESET_VALUE: u32 = 0;
}
