/// TODO: docs
#[derive(Copy, Clone)]
pub struct BufferId(nvim::BufHandle);

impl From<&nvim::api::Buffer> for BufferId {
    #[inline]
    fn from(buf: &nvim::api::Buffer) -> Self {
        Self(unsafe { core::mem::transmute_copy(buf) })
    }
}
