#[doc = "Register `HASH_LENGTH_IN_H` writer"]
pub type W = crate::W<HASH_LENGTH_IN_H_SPEC>;
#[doc = "Field `LENGTH_IN` writer - LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASH_LENGTH_IN_L needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the rfd_in bit of the HASH_IO_BUF_CTRL is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
pub type LENGTH_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - LENGTH_IN\\[63:32\\]
Message length registers. The content of these registers is used by the hash engine during the message padding phase of the hash session. The data lines of this registers are directly connected to the interface of the hash engine. For a write operation by the host, these registers should be written with the message length in bits. Final hash operations: The total input data length must be programmed for new hash operations that require finalization (padding). The input data must be provided through the slave or DMA interface. Continued hash operations (finalized): For continued hash operations that require finalization, the total message length must be programmed, including the length of previously hashed data that corresponds to the written input digest. Non-final hash operations: For hash operations that do not require finalization (input data length is multiple of 512-bits which is SHA-256 data block size), the length field does not need to be programmed since not used by the operation. If the message length in bits is below (2^32-1), then only HASH_LENGTH_IN_L needs to be written. The hardware automatically sets HASH_LENGTH_IN_H to 0s in this case. The host may write the length register at any time during the hash session when the rfd_in bit of the HASH_IO_BUF_CTRL is high. The length register must be written before the last data of the active hash session is written into the hash engine. host read operations from these register locations will return 0s. Note: When getting data from DMA, this register must be programmed before DMA is programmed to start."]
    #[inline(always)]
    #[must_use]
    pub fn length_in(&mut self) -> LENGTH_IN_W<HASH_LENGTH_IN_H_SPEC> {
        LENGTH_IN_W::new(self, 0)
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
#[doc = "Hash length register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_length_in_h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_LENGTH_IN_H_SPEC;
impl crate::RegisterSpec for HASH_LENGTH_IN_H_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_length_in_h::W`](W) writer structure"]
impl crate::Writable for HASH_LENGTH_IN_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_LENGTH_IN_H to value 0"]
impl crate::Resettable for HASH_LENGTH_IN_H_SPEC {
    const RESET_VALUE: u32 = 0;
}
