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
    $($fn_name:ident,)*
  } => {
    #[allow(clippy::manual_non_exhaustive)]
    pub struct $struct_name {
      $(pub $fn_name: usize,)*
    }

    impl $struct_name {
    #[allow(unreachable_code)]
      pub fn open () -> Result<(), ()> {
        let once = crate::link::MyOnce::new();

        // THIS IS THE SLOWDOWN
        once.get_or_try_init(|| {
            let _funcs = $struct_name {
              // without the question mark and the Ok the slowdown disappears
              // adding type annotation reduces the slowdown a lot
              // in the original example the type here was fully specified, this is also why
              // the regression now is worse than in the crater run
              // especially the err type is very perf relevant
              $($fn_name: Result::<usize, _>::Ok(usize::MAX)?,)*
            };

            Ok(())
        })?;
        Ok(())
      }
    }
  };
}
