use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, console};

pub trait Node {
  fn render(&self, element: &Element);
}

pub struct TextNode {
  pub text: String,
}

impl Node for TextNode {
  fn render(&self, element: &Element) {
    element.set_text_content(Some(self.text.as_str()));
  }
}

pub struct VNode {
  pub tag_name: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<Box<dyn Node>>
}

impl Node for VNode {
  fn render(&self, element: &Element) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let node = document.create_element(&self.tag_name).unwrap();
    element.append_child(&node).unwrap();

    for (k, v) in self.attributes.clone().into_iter() {
      element.set_attribute(k.as_str(), v.as_str()).unwrap();
    }

    for i in 0..self.children.len() {
      let c = &self.children[i];
      c.render(&node);
    }
    console::log_1(&"Finished constructing DOM Tree from VDomTree".into());
  }
}


pub struct VDomTree {
  pub root: Option<Box<dyn Node>>,
}

impl VDomTree {
  pub fn render(&mut self, root_id: &str, el: Box<dyn Node>) {
    self.root = Some(el);

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let root = document.get_element_by_id(root_id).expect("Expected root_id to be available inside <body> tag");

    self.root.as_ref().unwrap().render(&root);
  }
  
  pub fn create_element(tag_name: String, attributes: Option<HashMap<String, String>>, children: Option<Vec<Box <dyn Node>>>) -> Box<dyn Node> {
    Box::new(VNode {
      tag_name,
      attributes: attributes.unwrap_or(HashMap::new() as HashMap<String, String>),
      children: children.unwrap_or(vec![])
    }) as Box<dyn Node>
  }

  pub fn text_node(content: String) -> Box<dyn Node> {
   Box::new(TextNode {
      text: content,
    }) as Box<dyn Node>
  }
}
