#[doc = "Register `TASKS_CTSTOP` writer"]
pub type W = crate::W<TasksCtstopSpec>;
#[doc = "Stop calibration timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCtstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCtstop> for bool {
    #[inline(always)]
    fn from(variant: TasksCtstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CTSTOP` writer - Stop calibration timer"]
pub type TasksCtstopW<'a, REG> = crate::BitWriter<'a, REG, TasksCtstop>;
impl<'a, REG> TasksCtstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCtstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop calibration timer"]
    #[inline(always)]
    pub fn tasks_ctstop(&mut self) -> TasksCtstopW<TasksCtstopSpec> {
        TasksCtstopW::new(self, 0)
    }
}
#[doc = "Stop calibration timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ctstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCtstopSpec;
impl crate::RegisterSpec for TasksCtstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_ctstop::W`](W) writer structure"]
impl crate::Writable for TasksCtstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CTSTOP to value 0"]
impl crate::Resettable for TasksCtstopSpec {
    const RESET_VALUE: u32 = 0;
}
