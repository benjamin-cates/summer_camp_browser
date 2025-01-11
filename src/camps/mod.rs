use crate::structs::SummerCamp;

mod epic;
mod csssa;
mod cosmos;
pub mod generators;
mod ucsb_research;
mod yygs;
mod boston_university;
mod usc;

pub fn all_camps() -> Vec<SummerCamp> {
    // Temporary list
    // SSP
    // UCI Brain camp
    // Anson Clark Scholars Program
    // Stanford Summer Humanities institute
    // UCI Ethics
    // Interlochen
    // UCLA Summer art camp
    // Georgetown hoya
    // 
    // https://education.ucdavis.edu/ysp-application
    // https://belinblank.education.uiowa.edu/students/sstp/
    // https://www.stonybrook.edu/commcms/garcia/summer_program/apply
    // https://www.mathcamp.org/admission/
    // https://www.cmu.edu/pre-college/academic-programs/sams.html
    // https://mathily.org
    // https://promys.org
    // https://spcs.stanford.edu/programs/stanford-university-mathematics-camp-sumac
    // https://ei.jhu.edu
    // https://iyws.clas.uiowa.edu/how-to-apply/summer-residential-program
    // https://iyws.clas.uiowa.edu/6-week-online-courses
    // https://www.launchx.com/programs
    // https://globalyouth.wharton.upenn.edu/
    // https://haas.berkeley.edu/business-academy/
    // https://michiganross.umich.edu/undergraduate/summer-business-academy#:~:text=The%20Michigan%20Ross%20Summer%20Business%20Academy%20gives%20rising%20high%20school,business%20student%20at%20Michigan%20Ross.
    // https://force.mccombs.utexas.edu/summerprograms/s/
    // https://precollege.usc.edu/usc-marshall-pre-college-programs/
    // https://summer.georgetown.edu/programs/SHS03/global-business-academy/how-to-apply
    // https://sites.dartmouth.edu/summerscholars/course/business-foundations/
    // https://precollege.sps.columbia.edu/programs/explore-courses?interests=566&term=All&format=All&status=All&related_program=All
    // https://bwsi.mit.edu/apply-now/
    // https://www.cmu.edu/pre-college/academic-programs/ai_scholars.html
    // https://oxfordsummercourses.com/courses/artificial-intelligence-cambridge-16-17-years#dates-prices
    // https://globalyouth.wharton.upenn.edu/programs-courses/data-science-academy/
    // https://hai.stanford.edu/stanford-ai4all
    // https://psjp.princeton.edu
    // https://universitycollege.tufts.edu/pre-college/browse/adventures-veterinary-medicine
    // https://vet.uga.edu/education/k-12-programs/vetcamp/
    // https://cals.ncsu.edu/vetpac/vetcamp/
    // https://education.ucdavis.edu/ysp-application
    let mut out = vec![
        epic::get_epic(),
        csssa::get_csssa(),
        yygs::get_yygs(),
        //ucsb_research::get_rmp(),
        ucsb_research::get_sra(),
        usc::get_usc_precollege(),
    ];
    // Todo: Get cosmos essays
    out.extend(cosmos::get_cosmos());
    out.extend(boston_university::get_bu());
    out
}