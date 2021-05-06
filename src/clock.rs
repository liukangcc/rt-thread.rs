use crate::include::rtdef::*;

static mut rt_tick: rt_tick_t = 0;

pub fn rt_system_tick_init() {

}

pub fn rt_tick_get() -> rt_tick_t {
    unsafe {
        rt_tick
    }
}

pub fn rt_tick_set(tick: rt_tick_t) {
    unsafe {
        rt_tick = tick;
    }
}

pub fn rt_tick_increase() {
    unsafe {
        rt_tick += 1;
    }
}

pub fn rt_tick_from_millsecond(ms: rt_int32_t) -> rt_tick_t {
    let mut tick:rt_tick_t = 0;
    if ms < 0
    {
        tick = RT_WAITING_FOREVER;
    } else {
        tick = RT_TICK_PER_SECOND * (ms / 1000);
        tick += (RT_TICK_PER_SECOND * (ms % 1000) + 999) / 1000;
    }

    tick
}