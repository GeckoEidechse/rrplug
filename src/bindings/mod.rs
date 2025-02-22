pub mod command;
pub mod convar;
pub mod cvar;
pub mod entity;
pub mod plugin_abi;
pub mod squirrelclasstypes;
pub mod squirreldatatypes;
pub mod unwraped;

#[repr(C)]
pub struct OffsetStructField<T, const U: usize> {
    _pad: [::std::os::raw::c_char; U],
    value: T,
}

impl<T, const U: usize> std::ops::Deref for OffsetStructField<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, const U: usize> std::ops::DerefMut for OffsetStructField<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, const U: usize> OffsetStructField<T, U> {
    pub fn get_inner(&self) -> &T {
        &self.value
    }

    pub fn get_inner_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Copy + Clone, const U: usize> OffsetStructField<T, U> {
    pub fn copy_inner(&self) -> T {
        self.value
    }
}
