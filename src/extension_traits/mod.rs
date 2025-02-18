//! Extension traits to enhance external types with useful methods.

pub use self::as_out::{
    AsOut,
};
mod as_out;
pub use self::as_uninit::{
    AsUninit,
};
mod as_uninit;

pub use self::manually_drop_mut::{
    ManuallyDropMut,
};
mod manually_drop_mut;

pub use self::maybe_uninit::{
    MaybeUninitExt,
};
mod maybe_uninit;

cfg_alloc! {
    pub use self::boxed::{
        BoxAssumeInit,
        BoxNewUninitSlice,
        BoxUninit,
    };
    mod boxed;

    pub use self::vec::{
        VecCapacity,
    };
    mod vec;
}

cfg_std! {
    pub use self::vec::{
        VecExtendFromReader,
    };
}
