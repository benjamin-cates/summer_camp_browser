use crate::structs::SummerCamp;

mod boston_university;
mod georgetown;
mod nc_state;
mod tufts;
mod carnegie_mellon;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    let mut vec: Vec<SummerCamp> = boston_university::get_all().collect();
    vec.push(carnegie_mellon::get());
    vec.push(georgetown::georgetown_academies());
    vec.push(nc_state::get());
    vec.push(tufts::get());
    vec.into_iter()
}