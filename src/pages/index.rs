use std::collections::HashMap;

use wasm_bindgen::{JsValue, convert::IntoWasmAbi};
use crate::virtual_dom::{VDomTree, VNode, Node};

pub fn text() -> Box<dyn Node> {
  
  let mut text_attributes: HashMap<String, String> = HashMap::new();
  text_attributes.insert("style".into(), "color: red;".into());

  return VDomTree::create_element("div".into(), None, Some(vec![
    VDomTree::create_element("p".into(), None, Some(vec![
      VDomTree::text_node("Text1".into())
    ])),

    VDomTree::create_element("button".into(), None, Some(vec![
      VDomTree::text_node("Button1".into())
    ]))

  ]));
}

pub fn render()  -> Result<(), JsValue> {
  let mut vdom = VDomTree { root: None };
  vdom.render("root", text());

  Ok(())
}