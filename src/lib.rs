//! A simple way to call invoke in [tauri](https://crates.io/crates/tauri) from rust.
//!
//! ```rust
//! // calls 'tauri_invoke'
//! let future = invoke! {
//!     tauri_invoke -> String,
//!     foo: 1.0,
//!     bar: false,
//! };
//! ```

use wasm_bindgen::prelude::*;

#[doc(hidden)]
pub use js_sys;
#[doc(hidden)]
pub use wasm_bindgen;
#[doc(hidden)]
pub use wasm_bindgen_futures;

#[cfg(feature = "yew")]
#[doc(hidden)]
pub use yew;

#[wasm_bindgen]
extern "C" {
    #[doc(hidden)]
    #[wasm_bindgen(js_namespace = window)]
    pub fn __TAURI_INVOKE__(name: &str, arguments: JsValue) -> JsValue;
}

/// # Examples
/// Here we use [`invoke`] to call an invoke called `example_invoke`
/// with the arguments `foo` and `bar`.
/// Here `future` is a [`Future`](std::future::Future) that resolves to a [`String`].
/// ```rust
/// // calls 'tauri_invoke'
/// let future = invoke! {
///     example_invoke -> String,
///     foo: 1.0,
///     bar: false,
/// };
/// ```
#[macro_export]
macro_rules! invoke {
    {
        $name:ident -> $ty:ty $(,)?
        $(, $arg:ident : $val:expr),* $(,)?
    } => {async {
        let args = $crate::js_sys::Map::new();

        $(
            args.set(
                &$crate::wasm_bindgen::JsValue(::std::stringify!($arg)),
                &$crate::wasm_bindgen::JsValue::from_serde(&$val).unwrap()
            );
        )*

        let output = $crate::__TAURI_INVOKE__(
            ::std::stringify!($name),
            ::std::convert::From::from(args),
        );

        let promise = $crate::js_sys::Promise::from(output);
        let result = $crate::wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        result.into_serde::<$ty>().unwrap()
    }};
}

/// Use this macro to call an invoke from rust.
///
/// For more information see [`invoke`].
///
/// This macro should only be used in a context where a [`yew::hook`] should be used.
#[cfg(feature = "yew")]
#[macro_export]
macro_rules! use_invoke {
    {
        $name:ident -> $ty:ty $(,)?
        $(, $arg:ident : $val:expr)* $(,)?
    } => {{
        $crate::yew::use_future($crate::invoke! {
            $name -> $ty
            $(, $arg : $val)*
        })
    }};
}
