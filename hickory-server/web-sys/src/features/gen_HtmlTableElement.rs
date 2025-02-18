#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableElement , typescript_type = "HTMLTableElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub type HtmlTableElement;
    #[cfg(feature = "HtmlTableCaptionElement")]
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = caption)]
    #[doc = "Getter for the `caption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    pub fn caption(this: &HtmlTableElement) -> Option<HtmlTableCaptionElement>;
    #[cfg(feature = "HtmlTableCaptionElement")]
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = caption)]
    #[doc = "Setter for the `caption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    pub fn set_caption(this: &HtmlTableElement, value: Option<&HtmlTableCaptionElement>);
    #[cfg(feature = "HtmlTableSectionElement")]
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = tHead)]
    #[doc = "Getter for the `tHead` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn t_head(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;
    #[cfg(feature = "HtmlTableSectionElement")]
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = tHead)]
    #[doc = "Setter for the `tHead` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn set_t_head(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);
    #[cfg(feature = "HtmlTableSectionElement")]
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = tFoot)]
    #[doc = "Getter for the `tFoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn t_foot(this: &HtmlTableElement) -> Option<HtmlTableSectionElement>;
    #[cfg(feature = "HtmlTableSectionElement")]
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = tFoot)]
    #[doc = "Setter for the `tFoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    pub fn set_t_foot(this: &HtmlTableElement, value: Option<&HtmlTableSectionElement>);
    #[cfg(feature = "HtmlCollection")]
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = tBodies)]
    #[doc = "Getter for the `tBodies` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    pub fn t_bodies(this: &HtmlTableElement) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = rows)]
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    pub fn rows(this: &HtmlTableElement) -> HtmlCollection;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = align)]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn align(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = align)]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_align(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = border)]
    #[doc = "Getter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn border(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = border)]
    #[doc = "Setter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_border(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = frame)]
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn frame(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = frame)]
    #[doc = "Setter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_frame(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = rules)]
    #[doc = "Getter for the `rules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn rules(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = rules)]
    #[doc = "Setter for the `rules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_rules(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = summary)]
    #[doc = "Getter for the `summary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn summary(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = summary)]
    #[doc = "Setter for the `summary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_summary(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = width)]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn width(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = width)]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_width(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = bgColor)]
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn bg_color(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = bgColor)]
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_bg_color(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = cellPadding)]
    #[doc = "Getter for the `cellPadding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn cell_padding(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = cellPadding)]
    #[doc = "Setter for the `cellPadding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_cell_padding(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLTableElement" , js_name = cellSpacing)]
    #[doc = "Getter for the `cellSpacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn cell_spacing(this: &HtmlTableElement) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLTableElement" , js_name = cellSpacing)]
    #[doc = "Setter for the `cellSpacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn set_cell_spacing(this: &HtmlTableElement, value: &str);
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = createCaption)]
    #[doc = "The `createCaption()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn create_caption(this: &HtmlTableElement) -> HtmlElement;
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = createTBody)]
    #[doc = "The `createTBody()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn create_t_body(this: &HtmlTableElement) -> HtmlElement;
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = createTFoot)]
    #[doc = "The `createTFoot()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn create_t_foot(this: &HtmlTableElement) -> HtmlElement;
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = createTHead)]
    #[doc = "The `createTHead()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn create_t_head(this: &HtmlTableElement) -> HtmlElement;
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = deleteCaption)]
    #[doc = "The `deleteCaption()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_caption(this: &HtmlTableElement);
    # [wasm_bindgen (catch , method , structural , js_class = "HTMLTableElement" , js_name = deleteRow)]
    #[doc = "The `deleteRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_row(this: &HtmlTableElement, index: i32) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = deleteTFoot)]
    #[doc = "The `deleteTFoot()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_t_foot(this: &HtmlTableElement);
    # [wasm_bindgen (method , structural , js_class = "HTMLTableElement" , js_name = deleteTHead)]
    #[doc = "The `deleteTHead()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn delete_t_head(this: &HtmlTableElement);
    # [wasm_bindgen (catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow)]
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn insert_row(this: &HtmlTableElement) -> Result<HtmlElement, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "HTMLTableElement" , js_name = insertRow)]
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    pub fn insert_row_with_index(
        this: &HtmlTableElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
