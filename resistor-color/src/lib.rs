use int_enum::IntEnum;
use enum_iterator::{all, Sequence, IntoEnumIterator};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut int_color_vec = all::<ResistorColor>()
        .map(|c| c.int_value())
        .collect::<Vec<_>>();

    int_color_vec.sort();

    int_color_vec
        .iter()
        .map(|c| ResistorColor::from_int(*c).unwrap())
        .collect::<Vec<_>>()

}
