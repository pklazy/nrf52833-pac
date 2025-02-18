#[doc = "Register `PSELRESET[%s]` reader"]
pub type R = crate::R<PselresetSpec>;
#[doc = "Register `PSELRESET[%s]` writer"]
pub type W = crate::W<PselresetSpec>;
#[doc = "Field `PIN` reader - GPIO pin number onto which nRESET is exposed"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - GPIO pin number onto which nRESET is exposed"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT` reader - Port number onto which nRESET is exposed"]
pub type PortR = crate::BitReader;
#[doc = "Field `PORT` writer - Port number onto which nRESET is exposed"]
pub type PortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connect {
    #[doc = "1: Disconnect"]
    Disconnected = 1,
    #[doc = "0: Connect"]
    Connected = 0,
}
impl From<Connect> for bool {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECT` reader - Connection"]
pub type ConnectR = crate::BitReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connect {
        match self.bits {
            true => Connect::Disconnected,
            false => Connect::Connected,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Connect::Disconnected
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Connect::Connected
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type ConnectW<'a, REG> = crate::BitWriter<'a, REG, Connect>;
impl<'a, REG> ConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Disconnected)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Connected)
    }
}
impl R {
    #[doc = "Bits 0:4 - GPIO pin number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Port number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPIO pin number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<PselresetSpec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 5 - Port number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<PselresetSpec> {
        PortW::new(self, 5)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> ConnectW<PselresetSpec> {
        ConnectW::new(self, 31)
    }
}
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)\n\nYou can [`read`](crate::Reg::read) this register and get [`pselreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselresetSpec;
impl crate::RegisterSpec for PselresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselreset::R`](R) reader structure"]
impl crate::Readable for PselresetSpec {}
#[doc = "`write(|w| ..)` method takes [`pselreset::W`](W) writer structure"]
impl crate::Writable for PselresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELRESET[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PselresetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
