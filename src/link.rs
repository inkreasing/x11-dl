// adding a generic to this type adds a tiny regression
pub struct MyOnce();

impl MyOnce {
    pub fn new() -> Self {
        todo!()
    }
    // removing the generic Err type removes the regression
    pub fn get_or_try_init<F, E>(&self, _f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<(), E>,
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
              // adding type annotation reduces the slowdown a lot
              // in the original example the type here was fully specified, this is also why
              // the regression now is worse than in the crater run
              $($fn_name: ::std::mem::transmute(Result::<_, _>::Ok(usize::MAX)?),)*
            };

            Ok(())
          }
        })?;
        Ok(())
      }
    }
  };
}
