// The javascript API: https://github.com/apvarun/toastify-js/blob/572517040fae6a7f8be4a99778dacda9c933db45/README.md
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_name = Toastify)]
  pub type Toast;

  #[wasm_bindgen(constructor, js_class = "Toastify")]
  pub fn new(config: &JsValue) -> Toast;

  #[wasm_bindgen(method, structural, js_class = "Toastify", js_name = showToast)]
  pub fn show_toast(this: &Toast);
}

pub fn show_congrats_toast(msg: &str) {
  let config = Object::new();
  Reflect::set(
    &config,
    &"text".into(),
    &msg.into(),
  )
  .ok();

  let toast = Toast::new(&config);
  toast.show_toast();
}
