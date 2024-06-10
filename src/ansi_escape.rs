use std::fmt::{Display, Formatter, Result};

macro_rules! generate_styling_functions {
    (
        $(#[$enum_attrs:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                #[code = $code:literal]
                $(#[$variant_attrs:meta])*
                $variant:ident
            ),*
            $(,)?
        }
    ) => {
        $(#[$enum_attrs])*
        $vis enum $enum_name {
            $(
                $(#[$variant_attrs])*
                $variant,
            )*
        }

        impl $enum_name {
            #[allow(dead_code)]
            pub fn code(&self) -> u8 {
                match self {
                    $(Self::$variant => $code,)*
                }
            }
        }

        impl $enum_name {
            $(
                #[allow(dead_code)]
                pub fn $variant() -> Self {
                    Self::$variant
                }
            )*
        }

        pub trait TextStyling: Display {
            $(
                fn $variant(self) -> Style<Self>
                where
                    Self: Sized,
                {
                    self.style($enum_name::$variant)
                }
            )*
        }

        impl<T: Display> TextStyling for T {}
    }
}

pub struct Style<T> {
    text: T,
    code: u8,
}

generate_styling_functions! {
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub enum StyleCode {
        #[code = 37]
        white,
        #[code = 90]
        gray,
        #[code = 91]
        red,
        #[code = 92]
        green,
        #[code = 93]
        yellow,
        #[code = 94]
        blue,
        #[code = 1]
        bold,
        #[code = 3]
        italic,
        #[code = 4]
        underline,
        #[code = 9]
        strikethrough,
        #[code = 107]
        bg_white,
    }
}

pub trait GeneratedTextStyling: Display + TextStyling {
    fn style(self, style_code: StyleCode) -> Style<Self>
    where
        Self: Sized,
    {
        Style {
            text: self,
            code: style_code.code(),
        }
    }
}

impl<T: Display> GeneratedTextStyling for T {}

impl<T: Display> Display for Style<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\x1b[{}m{}\x1b[0m", self.code, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_styling() {
        let text = "Hello, world!";

        let white = text.white();
        let gray = text.gray();
        let red = text.red();
        let green = text.green();
        let yellow = text.yellow();
        let blue = text.blue();
        let bold = text.bold();
        let italic = text.italic();
        let underline = text.underline();
        let strikethrough = text.strikethrough();
        let bg_white = text.bg_white();

        assert_eq!(format!("{}", white), "\x1b[37mHello, world!\x1b[0m");
        assert_eq!(format!("{}", gray), "\x1b[90mHello, world!\x1b[0m");
        assert_eq!(format!("{}", red), "\x1b[91mHello, world!\x1b[0m");
        assert_eq!(format!("{}", green), "\x1b[92mHello, world!\x1b[0m");
        assert_eq!(format!("{}", yellow), "\x1b[93mHello, world!\x1b[0m");
        assert_eq!(format!("{}", blue), "\x1b[94mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bold), "\x1b[1mHello, world!\x1b[0m");
        assert_eq!(format!("{}", italic), "\x1b[3mHello, world!\x1b[0m");
        assert_eq!(format!("{}", underline), "\x1b[4mHello, world!\x1b[0m");
        assert_eq!(format!("{}", strikethrough), "\x1b[9mHello, world!\x1b[0m");
        assert_eq!(format!("{}", bg_white), "\x1b[107mHello, world!\x1b[0m");
    }
}
