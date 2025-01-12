use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get() -> SummerCamp {
    SummerCamp {
        application_fee: Some(0.0),
        application_opens: Some("8/1/24"),
        deadline: Some("1/21/25"),
        description: vec![
            "VetCAMP (Veterinary Career Aptitude and Mentoring Program) is an intensive weeklong summer program for high school students at the UGA campus in Athens. At VetCAMP, students experience veterinary medicine – and will be involved in activities aimed at evaluating their skills and competitiveness as future veterinarians. The week is filled with visits and hands-on learning at our veterinary teaching hospital, diagnostic labs, and the Poultry Diagnostic and Research Center — and highlighted by a behind-the-scenes tour of a veterinary medicine field trip. In addition, campers learn about what vet school is all about – from admissions to classes, to the wide variety of career paths veterinary medicine has to offer! Applicants must be from the United States.",
        ],
        identifier: "University of Georgia VetCamp",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(1.0),
        link: Some("https://vet.uga.edu/education/k-12-programs/vetcamp/"),
        location: Some("Athens, Georgia"),
        organization: Some("University of Georgia College of Veterinary Medicine"),
        requirements: vec![
            Requirement::Custom("In grades 10-12 at the time of application"),
            Requirement::RequiredEssay("Answer three short essay questions in no longer than two pages. 1. How do you believe veterinarians contribute to the larger society? 2. What qualities would you hope to bring as an aspiring veterinarian? 3. Why should you be selected?", Limit::Unspecified),
            Requirement::RequiredEssay("Record an unlisted video of you answering these three essay questions. Video no more than 3 minutes", Limit::Unspecified),
            Requirement::RecommendationForm("teacher or counselor"),
            Requirement::Custom("Quality jpg headshot"),
            Requirement::UnofficialTranscript,
        ],
        tuition: Some(1000.0),
        notes: vec![],
        ..Default::default()
    }
}