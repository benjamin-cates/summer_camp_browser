use crate::structs::SummerCamp;

mod yygs;
mod upenn;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    let mut out: Vec<SummerCamp> = vec![];
    out.push(yygs::get_yygs());
    out.push(upenn::get_wharton_global_youth());
    out.into_iter()
}