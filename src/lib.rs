#![deny(clippy::all)]

use napi::{bindgen_prelude::{Buffer, ClassInstance}, Env, JsUnknown};

#[macro_use]
extern crate napi_derive;

// #[napi(object)]
// pub struct MyObject {
//   pub name: String,
//   pub name1: String,
// }

// #[napi]
// pub struct MyClass {
//   name: String,
//   name1: String,
// }

// #[napi]
// impl MyClass {
//   #[napi(getter)]
//   pub fn name(&self) -> &String {
//     &self.name
//   }

//   #[napi(setter)]
//   pub fn set_name(&mut self, name: String) {
//     self.name = name;
//   }
// }

// #[napi]
// pub fn create_object() -> MyObject {
//   MyObject {
//     name: "SyMind".to_string(),
//     name1: "SyMind".to_string(),
//   }
// }

// #[napi]
// pub fn create_class() -> MyClass {
//   MyClass {
//     name: "SyMind".to_string(),
//     name1: "SyMind".to_string(),
//   }
// }

// #[napi]
// pub fn create_json() -> String {
//   "{\"name\": \"SyMind\"}".to_string()
// }

// #[napi]
// pub fn create_buffer() -> Buffer {
//   "{\"name\": \"SyMind\"}".to_string().into()
// }

// #[napi]
// pub fn create_js_buffer(env: Env) -> napi::Result<JsUnknown> {
//   let buf = env.create_buffer_with_data("{\"name\": \"SyMind\"}".as_bytes().to_vec())?;
//   Ok(buf.into_raw().into_unknown())
// }

// #[napi]
// pub fn set_object(obj: MyObject) {
//   obj.name;
// }

// #[napi]
// pub fn set_class_instance(class: ClassInstance<MyClass>) {
//   class.name();
// }


#[napi(object)]
pub struct Object1 {
  pub f1: String,
}

#[napi]
pub fn create_object1() -> Object1 {
  Object1 {
    f1: "foo".to_string(),
  }
}

#[napi]
pub fn set_object1(val: Object1) {
  drop(val);
}

#[napi(object)]
pub struct Object2 {
  pub f1: String,
  pub f2: String,
}

#[napi]
pub fn create_object2() -> Object2 {
  Object2 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
  }
}

#[napi]
pub fn set_object2(val: Object2) {
  drop(val);
}

#[napi(object)]
pub struct Object3 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
}

#[napi]
pub fn create_object3() -> Object3 {
  Object3 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
  }
}

#[napi]
pub fn set_object3(val: Object3) {
  drop(val);
}

#[napi(object)]
pub struct Object4 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
}

#[napi]
pub fn create_object4() -> Object4 {
  Object4 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
  }
}

#[napi]
pub fn set_object4(val: Object4) {
  drop(val);
}

#[napi]
pub struct Class1 {
  f1: String,
}

#[napi]
impl Class1 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }
}

#[napi]
pub fn create_class1() -> Class1 {
  Class1 {
    f1: "foo".to_string(),
  }
}

#[napi]
pub fn set_class1(val: ClassInstance<Class1>) {
  drop(val);
}

#[napi]
pub struct Class2 {
  f1: String,
  f2: String,
}

#[napi]
impl Class2 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }
}

#[napi]
pub fn create_class2() -> Class2 {
  Class2 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
  }
}

#[napi]
pub fn set_class2(val: ClassInstance<Class2>) {
  drop(val);
}

#[napi]
pub struct Class3 {
  f1: String,
  f2: String,
  f3: String,
}

#[napi]
impl Class3 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }
}

#[napi]
pub fn create_class3() -> Class3 {
  Class3 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
  }
}

#[napi]
pub fn set_class3(val: ClassInstance<Class3>) {
  drop(val);
}

#[napi]
pub struct Class4 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
}

#[napi]
impl Class4 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }
}

#[napi]
pub fn create_class4() -> Class4 {
  Class4 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
  }
}

#[napi]
pub fn set_class4(val: ClassInstance<Class4>) {
  drop(val);
}
