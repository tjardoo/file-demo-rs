use std::fmt::{Display, Formatter, Result};

pub struct Style<T> {
    text: T,
    code: u8,
}

#[allow(dead_code)]
pub enum StyleCode {
    White,
    Gray,
    Red,
    Green,
    Yellow,
    Blue,
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

pub trait TextStyling: Display {
    fn style(self, style_code: StyleCode) -> Style<Self>
    where
        Self: Sized,
    {
        let code = match style_code {
            StyleCode::White => 37,
            StyleCode::Gray => 90,
            StyleCode::Red => 91,
            StyleCode::Green => 92,
            StyleCode::Yellow => 93,
            StyleCode::Blue => 94,
            StyleCode::Bold => 1,
            StyleCode::Italic => 3,
            StyleCode::Underline => 4,
            StyleCode::Strikethrough => 9,
        };

        Style { text: self, code }
    }

    fn white(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::White)
    }

    fn gray(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Gray)
    }

    fn red(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Red)
    }

    fn green(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Green)
    }

    fn yellow(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Yellow)
    }

    fn blue(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Blue)
    }

    fn bold(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Bold)
    }

    fn italic(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Italic)
    }

    fn underline(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Underline)
    }

    fn strikethrough(self) -> Style<Self>
    where
        Self: Sized,
    {
        self.style(StyleCode::Strikethrough)
    }
}

impl<T: Display> TextStyling for T {}

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
    }
}
