
use crate::structs::{Limit, Requirement, SummerCamp};

pub fn uc_davis_ysp() -> SummerCamp {
    SummerCamp {
        acceptance_rate: None,
        link: Some("https://education.ucdavis.edu/ysp-about"),
        description: vec![
            "The UC Davis Young Scholars Program is a summer residential research program designed to expose approximately 40 high-achieving high school students to the world of original research within the fields of the biological, agricultural, environmental and natural sciences.",
            "Participants in the UC Davis Young Scholars Program work one-on-one with research faculty and research groups in state-of-the-art laboratories for six weeks. Each student will work on an individual project and prepare a journal-quality paper and symposium presentation about their work.",
            "In addition to scientific research, the UC Davis Young Scholars Program strives to introduce participants to the climate and culture of living and learning on a university campus. Staff make every effort to model the experiences that participants will have during their first years of college.",
            "All participants will be enrolled in five units of University Group Study Credit. Assignments for the program include research notebooks, a written article of journal quality describing the research project and its conclusions, presentation of the individual project at a research symposium and presentation of research to students at the home high school. All work in the program is graded and credit will be awarded after home presentations are completed.",
        ],
        deadline: Some("3/15/25"),
        identifier: "UC Davis Young Scholars Program (YSP)",
        length_wk: Some(6.0),
        application_opens: Some("1/15/25"),
        requirements: vec![
            Requirement::RequiredEssay("Unknown", Limit::Unspecified),
            Requirement::RecommendationForm("science teacher"),
            Requirement::RecommendationForm("another teacher"),
            Requirement::UnofficialTranscript,
            Requirement::ActivityList("details unknown"),
            Requirement::GradeRange(11..=12),
            Requirement::AgeRange(16..=17),
        ],
        application_fee: Some(40.0),
        tuition: Some(7500.0),
        apply_link: Some("https://ysp-app.ucdavis.edu/app/main"),
        last_updated: Some("1/10/25"),
        location: Some("Sacramento, California"),
        organization: Some("University of California, Davis"),
        ..Default::default()
    }
}
