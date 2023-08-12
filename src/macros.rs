#[macro_export]
// Creates a strip of a given size which is centered horizontally.
macro_rules! strip {
    ($ui:ident, $width:literal, $height:literal, $contents:expr) => {
        vertical_strip!($ui, [$height], |mut strip| {
            strip.cell(|ui| {
                horizontal_strip!(ui, [auto, $width, auto], $contents);
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
// Creates strips that allow an item to be centered both horizontally and vertically.
macro_rules! centered_item {
    ($ui:ident, $contents:expr) => {
        vertical_strip!($ui, [auto, auto, auto], |mut strip| {
            strip.empty();
            strip.cell(|ui| {
                horizontal_strip!(ui, [auto, auto, auto], |mut strip| {
                    strip.empty();
                    strip.cell($contents);
                    strip.empty();
                });
            });
            strip.empty();
        });
    };
}

#[macro_export]
// Inteprets the shorthand syntax for strip bounds (into `Size`s).
macro_rules! bounds {
    (auto) => {
        Size::remainder()
    };
    ($size:expr) => {
        Size::exact($size)
    };
}
