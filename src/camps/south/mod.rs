use crate::structs::SummerCamp;

mod georgia;
mod anson_clark;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    [
        georgia::get(),
        anson_clark::get(),
    ].into_iter()
}