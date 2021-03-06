#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseErrorKind {
  Invalid,
  Empty,
}

macro_rules! parse_from_string_error {
  ($type_name:ident, $error_name:ident $(, $child_parse_error_name:ident )* ) => {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct $error_name {
      kind: ParseErrorKind
    }

    impl $error_name {
      pub fn invalid() -> Self {
        $error_name {
          kind: ParseErrorKind::Invalid
        }
      }

      pub fn empty() -> Self {
        $error_name {
          kind: ParseErrorKind::Empty
        }
      }
    }

    impl ::std::fmt::Display for $error_name {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use ::std::error::Error;
        self.description().fmt(f)
      }
    }

    impl ::std::error::Error for $error_name {
      fn description(&self) -> &str {
        use self::ParseErrorKind::*;
        match self.kind {
          Invalid => concat!("invalid ", stringify!($type_name), " literal"),
          Empty => concat!("cannot parse ", stringify!($type_name), " from empty string"),
        }
      }
    }

    impl From<::std::option::NoneError> for $error_name {
        fn from(_err: ::std::option::NoneError) -> Self {
          $error_name::invalid()
        }
    }

    $(
      impl From<$child_parse_error_name> for $error_name {
        fn from(err: $child_parse_error_name) -> Self {
          use self::ParseErrorKind::*;
          match err.kind {
            Invalid => $error_name::invalid(),
            Empty => $error_name::empty()
          }
        }
      }
    )*
  }
}

/// General error-types
parse_from_string_error!(Http, ParseHttpError, ParseRequestError);

/// Error for Content-type
parse_from_string_error!(Content, ParseContentError);

/// Error for Request-type
parse_from_string_error!(
  Request,
  ParseRequestError,
  ParseRequestMethodError,
  ParseContentError
);

parse_from_string_error!(RequestMethod, ParseRequestMethodError);

/// Error for Response-type
parse_from_string_error!(
  Response,
  ParseResponseError,
  ParseStatusCodeError,
  ParseContentError
);

parse_from_string_error!(StatusCode, ParseStatusCodeError);
