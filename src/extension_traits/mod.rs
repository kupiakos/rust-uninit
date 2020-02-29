//! Extension traits to enhance external types with useful methods.

pub use self::as_out::{
    AsOut,
};
mod as_out;

pub use self::manually_drop_mut::{
    ManuallyDropMut,
};
mod manually_drop_mut;

pub use self::maybe_uninit::{
    MaybeUninitExt,
};
mod maybe_uninit;

cfg_std! {
    pub use self::boxed::{
        BoxUninit,
        BoxAssumeInit,
    };
    mod boxed;

    pub use self::vec::{
        VecExtendFromReader,
        VecCapacity,
    };
    mod vec;
}
