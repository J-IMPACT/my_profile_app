use web_sys::window;

pub fn set_meta(title: &str, description: &str) {
    let document = window().unwrap().document().unwrap();
    document.set_title(title);
    if let Some(meta) = document.query_selector("meta[name='description']").unwrap() {
        meta.set_attribute("content", description).ok();
    }
}
