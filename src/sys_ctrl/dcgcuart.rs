#[doc = "Register `DCGCUART` reader"]
pub type R = crate::R<DCGCUART_SPEC>;
#[doc = "Register `DCGCUART` writer"]
pub type W = crate::W<DCGCUART_SPEC>;
#[doc = "Field `UART0` reader - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<DCGCUART_SPEC> {
        UART0_W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<DCGCUART_SPEC> {
        UART1_W::new(self, 1)
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
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCUART_SPEC;
impl crate::RegisterSpec for DCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcuart::R`](R) reader structure"]
impl crate::Readable for DCGCUART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcuart::W`](W) writer structure"]
impl crate::Writable for DCGCUART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGCUART to value 0"]
impl crate::Resettable for DCGCUART_SPEC {
    const RESET_VALUE: u32 = 0;
}
