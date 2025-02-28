use crate::structs::SummerCamp;

mod uiowa;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    let mut out = vec![];
    out.push(uiowa::get_sstp());
    out.push(uiowa::get_young_writers());
    out.into_iter()
}