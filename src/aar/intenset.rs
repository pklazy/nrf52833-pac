#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Write '1' to enable interrupt for event END"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::Disabled,
            true => End::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == End::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == End::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to enable interrupt for event END"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolved {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Resolved> for bool {
    #[inline(always)]
    fn from(variant: Resolved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLVED` reader - Write '1' to enable interrupt for event RESOLVED"]
pub type ResolvedR = crate::BitReader<Resolved>;
impl ResolvedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resolved {
        match self.bits {
            false => Resolved::Disabled,
            true => Resolved::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Resolved::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Resolved::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolvedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ResolvedWO> for bool {
    #[inline(always)]
    fn from(variant: ResolvedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLVED` writer - Write '1' to enable interrupt for event RESOLVED"]
pub type ResolvedW<'a, REG> = crate::BitWriter<'a, REG, ResolvedWO>;
impl<'a, REG> ResolvedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ResolvedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event NOTRESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notresolved {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Notresolved> for bool {
    #[inline(always)]
    fn from(variant: Notresolved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRESOLVED` reader - Write '1' to enable interrupt for event NOTRESOLVED"]
pub type NotresolvedR = crate::BitReader<Notresolved>;
impl NotresolvedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Notresolved {
        match self.bits {
            false => Notresolved::Disabled,
            true => Notresolved::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Notresolved::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Notresolved::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event NOTRESOLVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotresolvedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<NotresolvedWO> for bool {
    #[inline(always)]
    fn from(variant: NotresolvedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRESOLVED` writer - Write '1' to enable interrupt for event NOTRESOLVED"]
pub type NotresolvedW<'a, REG> = crate::BitWriter<'a, REG, NotresolvedWO>;
impl<'a, REG> NotresolvedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(NotresolvedWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RESOLVED"]
    #[inline(always)]
    pub fn resolved(&self) -> ResolvedR {
        ResolvedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event NOTRESOLVED"]
    #[inline(always)]
    pub fn notresolved(&self) -> NotresolvedR {
        NotresolvedR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<IntensetSpec> {
        EndW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RESOLVED"]
    #[inline(always)]
    pub fn resolved(&mut self) -> ResolvedW<IntensetSpec> {
        ResolvedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event NOTRESOLVED"]
    #[inline(always)]
    pub fn notresolved(&mut self) -> NotresolvedW<IntensetSpec> {
        NotresolvedW::new(self, 2)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
