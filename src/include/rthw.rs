/// Basic error type for the library.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]

pub enum RT_HW_CACHE_OPS {
    RT_HW_CACHE_FLUSE = 0x01,
    RT_HW_CACHE_INVALIDATE = 0x02,
}