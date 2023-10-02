macro_rules! mk_color_type {
  ($ty:ident : $field_ty:ty, $($field_name:ident),*) => {
    #[repr(C)]
    #[derive(Clone, Debug, PartialEq)]
    pub struct $ty {
      $(
        pub $field_name: $field_ty
      ),*
    }

		impl $ty {
      pub fn new($($field_name: $field_ty),*) -> Self {
        Self { $($field_name),* }
      }
		}
  }
}

mk_color_type!(RGB: u8, r, g, b);
mk_color_type!(RGBA: u8, r, g, b, a);
mk_color_type!(RGB32F: f32, r, g, b);
mk_color_type!(RGBA32F: f32, r, g, b, a);
