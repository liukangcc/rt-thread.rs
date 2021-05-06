
/* RT-Thread version information */
#[macro_export]
macro_rules! RT_VERSION {
    () => {
        4
    };
}

#[macro_export]
macro_rules! RT_SUBVERSION {
    () => {
        0
    };
}

#[macro_export]
macro_rules! RT_REVISION {
    () => {
        3
    };
}

/* RT-Thread version */

/* RT-Thread basic data type definitions */
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_int8_t = i8;
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_int16_t = i16;
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_int32_t = i32;
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_uint8_t = u8;
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_uint16_t = u16;
#[cfg(not(RT_USING_ARCH_DATA_TYPE))]
pub type rt_uint32_t = u32;

#[cfg(ARCH_CPU_64BIT)]
pub type rt_int64_t = i32;
#[cfg(ARCH_CPU_64BIT)]
pub type rt_uint64_t = u32;

#[cfg(not(ARCH_CPU_64BIT))]
pub type rt_int64_t = i64;
#[cfg(not(ARCH_CPU_64BIT))]
pub type rt_uint64_t = u64;

pub type rt_bool_t = i32;
pub type rt_base_t = i64;
pub type rt_ubase_t = u64;
pub type rt_err_t = rt_base_t;
pub type rt_time_t = rt_uint32_t;
pub type rt_tick_t = rt_uint32_t;
pub type rt_flag_t = rt_base_t;
pub type rt_size_t = rt_ubase_t;
pub type rt_dev_t = rt_ubase_t;
pub type rt_off_t = rt_base_t;

/* boolean type definitions */
#[macro_export]
macro_rules! RT_TRUE {
    () => {
        1
    };
}

#[macro_export]
macro_rules! RT_FALSE {
    () => {
        0
    };
}


#[macro_export]
macro_rules! RT_WAITING_FOREVER {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! RT_WAITING_NO {
    () => {
        0
    };
}