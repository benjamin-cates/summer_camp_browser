use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get_garcia() -> SummerCamp {
    SummerCamp {
        application_fee: Some(50.0),
        deadline: Some("3/3/25"),
        description: vec![
            "The Research Scholar Program for High School Students offers the opportunity for \
             high school teachers and students to perform research on the forefronts of polymer \
             science and technology together with Garcia faculty and staff. Students work as part \
             of focused research teams and are taught to make original contributions of interest \
             to the scientific community. In addition to entering national competitions, the \
             students are encouraged to publish in refereed scientific journals and to present \
             their results at national conferences. Our goal is to convey to the students the \
             excitement we enjoy daily in research. The program has no set time limits. Research \
             is a lifetime learning experience, and we hope to remain a resource to our students \
             long after \"graduation\".",
        ],
        identifier: "Stony Brook Garcia Summer Research Program",
        last_updated: Some("1/18/25"),
        length_wk: Some(6.0),
        link: Some("https://www.stonybrook.edu/commcms/garcia/summer_program/about"),
        location: Some("Stony Brook, New York"),
        organization: Some("Stony Brook University"),
        requirements: vec![
            Requirement::MinUnweightedGPA(3.8),
            Requirement::AgeRange(16..=19),
            Requirement::GradeRange(11..=13),
            Requirement::TestScores("Above 60%. PSAT, SAT, ACT, or AP"),
            Requirement::OfficialTranscript,
            Requirement::RequiredEssay(
                "Please submit a single PDF document answering the following questions: 1. Please \
                 explain your reasons for wanting to participate in the Garcia Center Summer \
                 Scholars Research Program. 2. List all previous research experience, including \
                 research-oriented courses you may have or are currently taking. 3. If you had \
                 the opportunity to do research in any area of engineering, what would you do?",
                Limit::Unspecified,
            ),
            Requirement::ActivityList(
                "Short-typed Resume of Activities, Awards, and/or Scholarships (Maximum 2 pages)",
            ),
            Requirement::Custom(
                "Optional: Ancillary Material Upload (College Transcripts, Publications, etc.) \
                 (Maximum 4 pages)",
            ),
        ],
        notes: vec![],
        ..Default::default()
    }
}
