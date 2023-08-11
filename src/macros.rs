#[macro_export]
// Creates a strip of a given size which is centered horizontally.
macro_rules! strip {
    ($ui:ident, $width:literal, $height:literal, $contents:expr) => {
        vertical_strip!($ui, [$height], |mut strip| {
            strip.cell(|ui| {
                horizontal_strip!(ui, [remainder, $width, remainder], $contents);
            });
        });
    };
}

#[macro_export]
// Creates a vertical strip supporting a variable number of cells.
macro_rules! vertical_strip {
    ($ui:ident, [$($size:tt),*], $contents:expr) => {
        StripBuilder::new($ui)
            $(.size(bounds!($size)))*
            .vertical($contents)
    };
}

#[macro_export]
// Creates a horizontal strip supporting a variable number of cells.
macro_rules! horizontal_strip {
    ($ui:ident, [$($size:tt),*], $contents:expr) => {
        StripBuilder::new($ui)
            $(.size(bounds!($size)))*
            .horizontal($contents)
    };
}

#[macro_export]
// Inteprets the shorthand syntax for strip bounds (into `Size`s).
macro_rules! bounds {
    (remainder) => {
        Size::remainder()
    };
    ($size:expr) => {
        Size::exact($size)
    }; // ($size:literal) => {
       //     Size::exact($size)
       // };
}
