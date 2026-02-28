// JS bindings placeholder.
//
// The primary Leaflet integration is done via `js_sys::eval()` inside
// `components::map` — this is the simplest and most reliable approach
// because Leaflet's API is heavily method-chained and not well-suited
// to individual wasm_bindgen extern declarations.
//
// If finer-grained control is needed later, individual bindings can be
// added here with `#[wasm_bindgen]` extern blocks.
