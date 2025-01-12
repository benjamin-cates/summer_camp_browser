use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get_bbay() -> SummerCamp {
    SummerCamp {
        application_fee: Some(80.0),
        application_opens: Some("1/17/25"),
        deadline: Some("Rolling 3/1/25"),
        description: vec![
            "We look for academically motivated, mature students who are interested in attending \
             a rigorous college-prep summer business academy. B-BAY is for youth who want to \
             develop their knowledge of business or have a passion for business. The application \
             and accompanying materials should convey your level of maturity and motivation. \
             Essays are evaluated for writing and critical thinking ability, skill in organizing \
             and presenting thoughts, and the relevance of your answer to the questions posed.",
        ],
        requirements: vec![
            Requirement::RequiredEssay(
                "Answer all prompts in a single essay. Question 1: One of the defining principles \
                 of Berkeley Haas is “Beyond Yourself.” What does this principle mean to you? \
                 Question 2: Share an experience working on a team where you went “Beyond \
                 Yourself.” Why was this experience impactful, and what did you learn about \
                 yourself and working in a team? Question 3: How will participating in the \
                 Berkeley Business Academy for Youth help you achieve your academic and \
                 professional goals?)",
                Limit::Words(0..=500),
            ),
            Requirement::RecommendationForm("teacher of a core subject"),
            Requirement::RecommendationForm("anyone"),
            Requirement::UnofficialTranscript,
        ],
        identifier: "Berkeley Haas Business Academy for Youth (BBAY)",
        last_updated: Some("1/11/25"),
        length_wk: Some(2.0),
        link: Some("https://haas.berkeley.edu/business-academy/"),
        location: Some("Berkeley, California"),
        organization: Some("Berkeley Haas School of Business"),
        tuition: Some(6292.0),
        ..Default::default()
    }
}
