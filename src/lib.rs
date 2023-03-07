//! A simple way to call invoke in [tauri](https://crates.io/crates/tauri) from rust.
//!
//! ```rust
//! // define an invoke
//! invoke!(async fn example_invoke(foo: f32, bar: bool) -> String);
//!
//! // call the invoke
//! let future = example_invoke(1.0, false);
//! ```

use wasm_bindgen::prelude::*;

#[doc(hidden)]
pub use js_sys;
#[doc(hidden)]
pub use serde;
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

#[doc(hidden)]
pub type InvokeResult<T = ()> = Result<T, String>;

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
        $vis async fn $name($($arg: $arg_ty),*) -> $crate::InvokeResult$(<$ty>)? {
            let args = $crate::js_sys::Object::new();
            let serializer = $crate::serde_wasm_bindgen::Serializer::json_compatible();

            $(
                $crate::js_sys::Reflect::set(
                    &args,
                    &$crate::wasm_bindgen::JsValue::from_str(::std::stringify!($arg)),
                    &$crate::serde::Serialize::serialize(&$arg, &serializer).unwrap(),
                ).expect("failed to set argument");
            )*

            let output = $crate::__TAURI_INVOKE__(
                ::std::stringify!($name),
                ::std::convert::From::from(args),
            );

            let promise = $crate::js_sys::Promise::from(output);
            let result = $crate::wasm_bindgen_futures::JsFuture::from(promise).await;

            Ok(match result {
                #[allow(unused_variables)]
                ::std::result::Result::Ok(value) => {$(
                    $crate::serde_wasm_bindgen::from_value::<$ty>(value)
                        .expect("Failed to deserialize output of invoke, maybe the type is wrong?")
                )?},
                ::std::result::Result::Err(err) => {
                    return ::std::result::Result::Err(err.as_string().unwrap());
                }
            })
        }
    )*};
}
