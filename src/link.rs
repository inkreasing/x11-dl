pub struct MyOnce<T>(std::marker::PhantomData<T>);

impl<T> MyOnce<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn get_or_try_init<F, E>(&self, _f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        todo!()
    }
}

macro_rules! x11_link {
  { $struct_name:ident,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
  } => {
    #[allow(clippy::manual_non_exhaustive)]
    #[allow(improper_ctypes_definitions)]
    pub struct $struct_name {
      $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
    }

    impl $struct_name {
    #[allow(unreachable_code)]
      pub fn open () -> Result<(), ()> {
        let once = crate::link::MyOnce::new();

        // THIS IS THE SLOWDOWN
        once.get_or_try_init(|| {
          unsafe {
            let _funcs = $struct_name {
              // without the question mark and the Ok the slowdown disappears
              $($fn_name: ::std::mem::transmute(Ok(usize::MAX)?),)*
            };

            Ok(())
          }
        })?;
        Ok(())
      }
    }
  };
}
