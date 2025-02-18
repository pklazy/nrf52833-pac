#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
    Master = 0,
    #[doc = "1: Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
    Slave = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - I2S mode."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Master,
            true => Mode::Slave,
        }
    }
    #[doc = "Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Mode::Master
    }
    #[doc = "Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Mode::Slave
    }
}
#[doc = "Field `MODE` writer - I2S mode."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Master)
    }
    #[doc = "Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Slave)
    }
}
impl R {
    #[doc = "Bit 0 - I2S mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<ModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "I2S mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
