use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get() -> SummerCamp {
    SummerCamp {
        application_fee: Some(56.0),
        application_opens: Some("1/31/25"),
        deadline: Some("unknown"),
        description: vec![
            "VetCAMP is a 5 day summer camp hosted by the Veterinary Professions Advising Center (VetPAC) at North Carolina State University. Since 2011, this camp has educated students about careers in veterinary medicine and the various opportunities within the field. Camp activities include a tour of the College of Veterinary Medicine, basics of a small animal clinic, hands-on experience with horses, sheep and pigs at the NC State Animal Education Units,  suture and dissection labs, learning about NC State majors and more!",
        ],
        identifier: "NC State VetCamp",
        keywords: vec![("veterinary", 10), ("veterinarian", 10)],
        last_updated: Some("1/11/25"),
        length_wk: Some(1.0),
        link: Some("https://cals.ncsu.edu/vetpac/vetcamp/"),
            location: Some("Raleigh, North Carolina"),
        organization: Some("North Carolina State University"),
        requirements: vec![
            Requirement::AgeRange(16..=18),
            Requirement::ActivityList("Unknown"),
            Requirement::RequiredEssay("Unknown", Limit::Unspecified),
        ],
        tuition: Some(885.0),
        notes: vec![],
        ..Default::default()
    }
}