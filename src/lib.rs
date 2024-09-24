#![deny(clippy::all)]

use napi::bindgen_prelude::ClassInstance;
use napi_derive::*;

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
pub fn set_object1(_val: Object1) {
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
pub fn set_object2(_val: Object2) {
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
pub fn set_object3(_val: Object3) {
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
pub fn set_object4(_val: Object4) {
}

#[napi(object)]
pub struct Object5 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
}

#[napi]
pub fn create_object5() -> Object5 {
  Object5 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
  }
}

#[napi]
pub fn set_object5(_val: Object5) {
}

#[napi(object)]
pub struct Object6 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
  pub f6: String,
}

#[napi]
pub fn create_object6() -> Object6 {
  Object6 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
  }
}

#[napi]
pub fn set_object6(_val: Object6) {
}

#[napi(object)]
pub struct Object7 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
  pub f6: String,
  pub f7: String,
}

#[napi]
pub fn create_object7() -> Object7 {
  Object7 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
  }
}

#[napi]
pub fn set_object7(_val: Object7) {
}

#[napi(object)]
pub struct Object8 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
  pub f6: String,
  pub f7: String,
  pub f8: String,
}

#[napi]
pub fn create_object8() -> Object8 {
  Object8 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
  }
}

#[napi]
pub fn set_object8(_val: Object8) {
}

#[napi(object)]
pub struct Object9 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
  pub f6: String,
  pub f7: String,
  pub f8: String,
  pub f9: String,
}

#[napi]
pub fn create_object9() -> Object9 {
  Object9 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
    f9: "foo".to_string(),
  }
}

#[napi]
pub fn set_object9(_val: Object9) {
}

#[napi(object)]
pub struct Object10 {
  pub f1: String,
  pub f2: String,
  pub f3: String,
  pub f4: String,
  pub f5: String,
  pub f6: String,
  pub f7: String,
  pub f8: String,
  pub f9: String,
  pub f10: String,
}

#[napi]
pub fn create_object10() -> Object10 {
  Object10 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
    f9: "foo".to_string(),
    f10: "foo".to_string(),
  }
}

#[napi]
pub fn set_object10(_val: Object10) {
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

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }
}

#[napi]
pub fn create_class1() -> Class1 {
  Class1 {
    f1: "foo".to_string(),
  }
}

#[napi]
pub fn set_class1(_val: ClassInstance<Class1>) {
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

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
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
pub fn set_class2(_val: ClassInstance<Class2>) {
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

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
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
pub fn set_class3(_val: ClassInstance<Class3>) {
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

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
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
pub fn set_class4(_val: ClassInstance<Class4>) {
}

#[napi]
pub struct Class5 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
}

#[napi]
impl Class5 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f5 = val;
  }
}

#[napi]
pub fn create_class5() -> Class5 {
  Class5 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
  }
}

#[napi]
pub fn set_class5(_val: ClassInstance<Class5>) {
}

#[napi]
pub struct Class6 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
  f6: String,
}

#[napi]
impl Class6 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f6(&self) -> &String {
    &self.f6
  }

  #[napi(setter)]
  pub fn set_f6(&mut self, val: String) {
    self.f1 = val;
  }
}

#[napi]
pub fn create_class6() -> Class6 {
  Class6 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
  }
}

#[napi]
pub fn set_class6(_val: ClassInstance<Class6>) {
}

#[napi]
pub struct Class7 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
  f6: String,
  f7: String,
}

#[napi]
impl Class7 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f5 = val;
  }

  #[napi(getter)]
  pub fn f6(&self) -> &String {
    &self.f6
  }

  #[napi(setter)]
  pub fn set_f6(&mut self, val: String) {
    self.f6 = val;
  }

  #[napi(getter)]
  pub fn f7(&self) -> &String {
    &self.f7
  }

  #[napi(setter)]
  pub fn set_f7(&mut self, val: String) {
    self.f7 = val;
  }
}

#[napi]
pub fn create_class7() -> Class7 {
  Class7 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
  }
}

#[napi]
pub fn set_class7(_val: ClassInstance<Class7>) {
}

#[napi]
pub struct Class8 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
  f6: String,
  f7: String,
  f8: String,
}

#[napi]
impl Class8 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f5 = val;
  }

  #[napi(getter)]
  pub fn f6(&self) -> &String {
    &self.f6
  }

  #[napi(setter)]
  pub fn set_f6(&mut self, val: String) {
    self.f6 = val;
  }

  #[napi(getter)]
  pub fn f7(&self) -> &String {
    &self.f7
  }

  #[napi(setter)]
  pub fn set_f7(&mut self, val: String) {
    self.f7 = val;
  }

  #[napi(getter)]
  pub fn f8(&self) -> &String {
    &self.f8
  }

  #[napi(setter)]
  pub fn set_f8(&mut self, val: String) {
    self.f8 = val;
  }
}

#[napi]
pub fn create_class8() -> Class8 {
  Class8 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
  }
}

#[napi]
pub fn set_class8(_val: ClassInstance<Class8>) {
}

#[napi]
pub struct Class9 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
  f6: String,
  f7: String,
  f8: String,
  f9: String,
}

#[napi]
impl Class9 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f5 = val;
  }

  #[napi(getter)]
  pub fn f6(&self) -> &String {
    &self.f6
  }

  #[napi(setter)]
  pub fn set_f6(&mut self, val: String) {
    self.f6 = val;
  }

  #[napi(getter)]
  pub fn f7(&self) -> &String {
    &self.f7
  }

  #[napi(setter)]
  pub fn set_f7(&mut self, val: String) {
    self.f7 = val;
  }

  #[napi(getter)]
  pub fn f8(&self) -> &String {
    &self.f8
  }

  #[napi(setter)]
  pub fn set_f8(&mut self, val: String) {
    self.f8 = val;
  }

  #[napi(getter)]
  pub fn f9(&self) -> &String {
    &self.f9
  }

  #[napi(setter)]
  pub fn set_f9(&mut self, val: String) {
    self.f9 = val;
  }
}

#[napi]
pub fn create_class9() -> Class9 {
  Class9 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
    f9: "foo".to_string(),
  }
}

#[napi]
pub fn set_class9(_val: ClassInstance<Class9>) {
}


#[napi]
pub struct Class10 {
  f1: String,
  f2: String,
  f3: String,
  f4: String,
  f5: String,
  f6: String,
  f7: String,
  f8: String,
  f9: String,
  f10: String,
}

#[napi]
impl Class10 {
  #[napi(getter)]
  pub fn f1(&self) -> &String {
    &self.f1
  }

  #[napi(setter)]
  pub fn set_f1(&mut self, val: String) {
    self.f1 = val;
  }

  #[napi(getter)]
  pub fn f2(&self) -> &String {
    &self.f2
  }

  #[napi(setter)]
  pub fn set_f2(&mut self, val: String) {
    self.f2 = val;
  }

  #[napi(getter)]
  pub fn f3(&self) -> &String {
    &self.f3
  }

  #[napi(setter)]
  pub fn set_f3(&mut self, val: String) {
    self.f3 = val;
  }

  #[napi(getter)]
  pub fn f4(&self) -> &String {
    &self.f4
  }

  #[napi(setter)]
  pub fn set_f4(&mut self, val: String) {
    self.f4 = val;
  }

  #[napi(getter)]
  pub fn f5(&self) -> &String {
    &self.f5
  }

  #[napi(setter)]
  pub fn set_f5(&mut self, val: String) {
    self.f5 = val;
  }

  #[napi(getter)]
  pub fn f6(&self) -> &String {
    &self.f6
  }

  #[napi(setter)]
  pub fn set_f6(&mut self, val: String) {
    self.f6 = val;
  }

  #[napi(getter)]
  pub fn f7(&self) -> &String {
    &self.f7
  }

  #[napi(setter)]
  pub fn set_f7(&mut self, val: String) {
    self.f7 = val;
  }

  #[napi(getter)]
  pub fn f8(&self) -> &String {
    &self.f8
  }

  #[napi(setter)]
  pub fn set_f8(&mut self, val: String) {
    self.f8 = val;
  }

  #[napi(getter)]
  pub fn f9(&self) -> &String {
    &self.f9
  }

  #[napi(setter)]
  pub fn set_f9(&mut self, val: String) {
    self.f9 = val;
  }

  #[napi(getter)]
  pub fn f10(&self) -> &String {
    &self.f10
  }

  #[napi(setter)]
  pub fn set_f10(&mut self, val: String) {
    self.f10 = val;
  }
}

#[napi]
pub fn create_class10() -> Class10 {
  Class10 {
    f1: "foo".to_string(),
    f2: "foo".to_string(),
    f3: "foo".to_string(),
    f4: "foo".to_string(),
    f5: "foo".to_string(),
    f6: "foo".to_string(),
    f7: "foo".to_string(),
    f8: "foo".to_string(),
    f9: "foo".to_string(),
    f10: "foo".to_string(),
  }
}

#[napi]
pub fn set_class10(_val: ClassInstance<Class10>) {
}

