//! Conditional trait implementations for external libraries.

/*
How do I support a new external library?

Let's say we want to add support for `my_library`.

First, we create a module under `external`, like `serde` with any specialized code.
Ideally, any utilities in here should just work off the `Flags` trait and maybe a
few other assumed bounds.

Next, re-export the library from the `__private` module here.

Next, define a macro like so:

```rust
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "serde")]
macro_rules! __impl_external_bitflags_my_library {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {
        // Implementation goes here
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "my_library"))]
macro_rules! __impl_external_bitflags_my_library {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {};
}
```

Note that the macro is actually defined twice; once for when the `my_library` feature
is available, and once for when it's not. This is because the `__impl_external_bitflags_my_library`
macro is called in an end-user's library, not in `bitflags`. In an end-user's library we don't
know whether or not a particular feature of `bitflags` is enabled, so we unconditionally call
the macro, where the body of that macro depends on the feature flag.

Now, we add our macro call to the `__impl_external_bitflags` macro body:

```rust
__impl_external_bitflags_my_library! {
    $InternalBitFlags: $T {
        $(
            $(#[$attr $($args)*])*
            $Flag;
        )*
    }
}
```
*/

pub(crate) mod __private {
    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(feature = "arbitrary")]
    pub use arbitrary;

    #[cfg(feature = "bytemuck")]
    pub use bytemuck;
}

#[cfg(feature = "serde")]
pub mod serde;

/// Implement `Serialize` and `Deserialize` for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "serde")]
macro_rules! __impl_external_bitflags_serde {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
                )*
        }
    ) => {
        impl $crate::__private::serde::Serialize for $InternalBitFlags {
            fn serialize<S: $crate::__private::serde::Serializer>(
                &self,
                serializer: S,
            ) -> $crate::__private::core::result::Result<S::Ok, S::Error> {
                $crate::serde::serialize(
                    self,
                    serializer,
                )
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for $InternalBitFlags {
            fn deserialize<D: $crate::__private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> $crate::__private::core::result::Result<Self, D::Error> {
                $crate::serde::deserialize(
                    deserializer,
                )
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
macro_rules! __impl_external_bitflags_serde {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
                )*
        }
    ) => {};
}

#[cfg(feature = "arbitrary")]
pub mod arbitrary;

#[cfg(feature = "bytemuck")]
mod bytemuck;

/// Implements traits from external libraries for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_external_bitflags {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident;
            )*
        }
    ) => {
        // Any new library traits impls should be added here
        // Use `serde` as an example: generate code when the feature is available,
        // and a no-op when it isn't

        __impl_external_bitflags_serde! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }

        __impl_external_bitflags_arbitrary! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }

        __impl_external_bitflags_bytemuck! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }
    };
}

/// Implement `Arbitrary` for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "arbitrary")]
macro_rules! __impl_external_bitflags_arbitrary {
    (
            $InternalBitFlags:ident: $T:ty {
                $(
                    $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
            }
    ) => {
        impl<'a> $crate::__private::arbitrary::Arbitrary<'a> for $InternalBitFlags {
            fn arbitrary(
                u: &mut $crate::__private::arbitrary::Unstructured<'a>,
            ) -> $crate::__private::arbitrary::Result<Self> {
                $crate::arbitrary::arbitrary(u)
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "arbitrary"))]
macro_rules! __impl_external_bitflags_arbitrary {
    (
            $InternalBitFlags:ident: $T:ty {
                $(
                    $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
            }
    ) => {};
}

/// Implement `Pod` and `Zeroable` for the internal bitflags type.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(feature = "bytemuck")]
macro_rules! __impl_external_bitflags_bytemuck {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
        }
    ) => {
        // SAFETY: $InternalBitFlags is guaranteed to have the same ABI as $T,
        // and $T implements Pod
        unsafe impl $crate::__private::bytemuck::Pod for $InternalBitFlags
        where
            $T: $crate::__private::bytemuck::Pod,
        {

        }

        // SAFETY: $InternalBitFlags is guaranteed to have the same ABI as $T,
        // and $T implements Zeroable
        unsafe impl $crate::__private::bytemuck::Zeroable for $InternalBitFlags
        where
            $T: $crate::__private::bytemuck::Zeroable,
        {

        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
#[cfg(not(feature = "bytemuck"))]
macro_rules! __impl_external_bitflags_bytemuck {
    (
        $InternalBitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                    $Flag:ident;
                )*
        }
    ) => {};
}
