use crate::structs::SummerCamp;

mod yygs;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    std::iter::once(yygs::get_yygs())
}