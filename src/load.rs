use wasm_bindgen::{JsCast, JsValue};
use aframers::browser::document;
use web_sys::HtmlScriptElement;
use wasm_bindgen::closure::Closure;

const INNER_DIV: &str = r#"
<div id="htmlElementWrapper"
     style="width: 512px; height: 512px; position: fixed; left: 0; top: 0; z-index: -2; overflow: hidden">
    <div id="htmlElement"
         style="width:100%; height:100%; background: #F8F8F8; color: #333; font-size: 48px">
        <img src="assets/sample.svg" style="width:100%; height:100%" alt="sample"/>
    </div>
</div>
"#;

pub fn load() -> Result<(), JsValue> {
	// Adding the htmlElementWrapper div before loading the shader script allows the Meta Quest browser
	// to draw the html-material starting from the first page load.  If the div is added after the
	// script has already loaded, the first page load renders the material as black and only renders it
	// correctly after the page is refreshed.
	let div = document().create_element("div")?;
	div.set_inner_html(INNER_DIV);
	document().body().expect("body").append_child(&div)?;

	let script = document().create_element("script")?;
	{
		let script: &HtmlScriptElement = &script.unchecked_ref();
		script.set_type("text/javascript");
		script.set_src("assets/updated-shader.js");
		script.add_event_listener_with_callback(
			"load",
			Closure::once_into_js(|| { crate::run::run().expect("run"); }).as_ref().unchecked_ref(),
		)?;
	}
	document().head().expect("head").append_child(&script)?;
	Ok(())
}