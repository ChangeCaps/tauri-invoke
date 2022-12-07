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
pub use serde_wasm_bindgen;
#[doc(hidden)]
pub use wasm_bindgen;
#[doc(hidden)]
pub use wasm_bindgen_futures;

#[wasm_bindgen]
extern "C" {
    #[doc(hidden)]
    #[wasm_bindgen(js_namespace = window)]
    pub fn __TAURI_INVOKE__(name: &str, arguments: JsValue) -> JsValue;
}

/// # Examples
/// ```rust
/// // define an invoke
/// invoke!(async fn example_invoke(foo: f32, bar: bool) -> String);
///
/// // call the invoke
/// let future = example_invoke(1.0, false);
/// ```
#[macro_export]
macro_rules! invoke {
    { $( $vis:vis async fn $name:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ty:ty)? ; )* } => {
        $crate::invoke!($( $vis async fn $name($($arg: $arg_ty),*) $(-> $ty)? ),*);
    };
    { $( $vis:vis async fn $name:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ty:ty)? ),* $(,)? } => {$(
        $vis async fn $name($($arg: $arg_ty),*) $(-> $ty)? {
            let args = $crate::js_sys::Map::new();

            $(
                args.set(
                    &$crate::wasm_bindgen::JsValue::from(::std::stringify!($arg)),
                    &$crate::serde_wasm_bindgen::to_value(&$arg).unwrap(),
                );
            )*

            let output = $crate::__TAURI_INVOKE__(
                ::std::stringify!($name),
                ::std::convert::From::from(args),
            );

            let promise = $crate::js_sys::Promise::from(output);
            let result = $crate::wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
            $crate::serde_wasm_bindgen::from_value(result)
                .expect("Failed to deserialize output of invoke, maybe the type is wrong?")
        }
    )*};
}
