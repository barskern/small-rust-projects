
#[macro_export]
macro_rules! parse_from_string_error {
  ($error_name:ident, $type_name:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct $error_name {
      kind: Option<()> // Use option to simulate custom enum
    }

    impl $error_name {
      fn invalid() -> Self {
        $error_name {
          kind: Some(())
        }
      }

      fn empty() -> Self {
        $error_name {
          kind: None
        }
      }
    }

    impl std::fmt::Display for $error_name {
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        self.description().fmt(f)
      }
    }

    impl std::error::Error for $error_name {
      fn description(&self) -> &str {
        match self.kind {
          Some(_) => concat!("invalid ", stringify!($type_name), " literal"),          
          None => concat!("cannot parse ", stringify!($type_name), " from empty string"),
          _ => unreachable!(),
        }
      }
    }
  }
}

