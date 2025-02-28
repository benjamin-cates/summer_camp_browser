use crate::structs::SummerCamp;
mod ssp;
pub fn get() -> impl Iterator<Item=SummerCamp> {
    let mut vec: Vec<SummerCamp> = vec![];
    vec.push(ssp::get());
    vec.into_iter()
}