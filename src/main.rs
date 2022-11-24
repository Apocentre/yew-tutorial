use yew_tutorial::view::core::app::App;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::start_app::<App>();
}
