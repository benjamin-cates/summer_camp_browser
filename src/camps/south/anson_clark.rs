use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get() -> SummerCamp {
    SummerCamp {
        application_fee: Some(25.0),
        deadline: Some("2/24/25"),
        description: vec![
            "The Clark Scholars Program is designed to attract gifted students from around the \
             nation and globe. The program allows students the opportunity to work hand-in-hand \
             with outstanding faculty on the general academic campus and the Health Science \
             Center in a research intensive setting. The seven-week program also includes weekly \
             seminars, discussions, and field trips. The students are afforded an atmosphere \
             designed to develop their critical thinking abilities and career interests with \
             faculty and other students like themselves. Students are selected on the basis of \
             their academic accomplishments, teacher recommendations and career objectives. Each \
             year, applicant pools are very competitive. For example, the average SAT score for \
             the 12 participants each summer is usually in the 99th percentile.",
        ],
        identifier: "Anson L. Clark Scholars",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        link: Some("https://www.depts.ttu.edu/clarkscholars/ProgramDetails.php"),
        location: Some("Lubbock, Texas"),
        organization: Some("Texas Tech University"),
        requirements: vec![
            Requirement::AgeRange(17..=19),
            Requirement::RequiredEssay(
                "Describe your primary research interest, being very specific. This will help \
                 with matching you to a research professor.",
                Limit::Characters(0..=2000),
            ),
            Requirement::RequiredEssay(
                "Optional - Describe your secondary research interest, being very specific.",
                Limit::Characters(0..=2000),
            ),
            Requirement::RequiredEssay(
                "Tell the Selection Committee about yourself and your reasons for applying to the \
                 Clark Scholarship Program. Please include your future goals and how becoming a \
                 Clark Scholar would contribute to achieving those goals.",
                Limit::Characters(0..=2000),
            ),
            Requirement::RequiredEssay(
                "Tell us about your most fulfilling service experience.",
                Limit::Characters(0..=1500),
            ),
            Requirement::UnofficialTranscript,
            Requirement::RecommendationForm("teacher"),
            Requirement::RecommendationForm("teacher"),
            Requirement::RecommendationForm("teacher or mentor"),
            Requirement::ActivityList(
                "List of Top 5 Activities - The student should list what they consider their most \
                 important involvment activities/accomplishments. These can include academic \
                 honors or awards, research programs, and other extracurricular activities in and \
                 outside of school. (Please note - The essay prompt about the applicant's most \
                 fulfilling service experience creates an opportunity to write about a sixth \
                 activity. Keep this in mind, as a resume or an additional list of activities is \
                 not allowed.) ",
            ),
            Requirement::TestScores("SAT or ACT (PSAT or PACT in case scores aren't out yet"),
            Requirement::TestScores("Optional AP scores"),
        ],
        tuition: Some(-750.0),
        ..Default::default()
    }
}
