use crate::structs::SummerCamp;

mod cosmos;
mod csssa;
mod epic;
mod uci;
mod ucsb_research;
mod usc;
mod uc_davis;
mod berkeley;
mod stanford;
mod ucla;

pub fn get() -> impl Iterator<Item=SummerCamp> {
    let mut vec: Vec<SummerCamp> = cosmos::get_cosmos().collect();
    vec.push(csssa::get_csssa());
    vec.push(epic::get_epic());
    vec.push(uci::uci_brain_camp());
    vec.push(uci::uci_ethics());
    vec.push(ucsb_research::get_sra());
    vec.push(usc::get_usc_precollege());
    vec.push(uc_davis::uc_davis_ysp());
    vec.push(berkeley::get_bbay());
    vec.push(stanford::get_humanities());
    vec.push(stanford::get_sumac());
    vec.push(stanford::get_summer_institutes());
    vec.push(ucla::get_institutes());
    vec.push(ucla::get_scip());
    vec.into_iter()
}