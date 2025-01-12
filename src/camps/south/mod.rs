use crate::structs::SummerCamp;

mod georgia;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    std::iter::once(georgia::get())
}