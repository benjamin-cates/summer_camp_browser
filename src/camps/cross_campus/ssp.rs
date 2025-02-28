use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get() -> SummerCamp {
    SummerCamp {
        application_fee: Some(0.0),
        deadline: Some("1/24/25"),
        description: vec![
            "The Summer Science Program offers teens an exhilarating and inspiring immersion into \
             hands-on experimental science. Working in teams of three, 36 or 24 participants and \
             8 faculty form a supportive “living and learning community” over 39 days. Each team \
             completes a real research project, taking and analyzing original data. Afterward, \
             they join a worldwide network of 3,700+ alumni of all ages. In 2024 we operated 12 \
             programs:",
            "five in Astrophysics: research in near-earth asteroid orbit determination at New \
             Mexico State Univ., Univ. of Colorado Boulder, Georgia College & State Univ., and \
             Univ. of North Carolina Chapel Hill.",
            "three in Biochemistry: research in fungal crop pathogens at Purdue Univ. and Indiana \
             Univ.",
            "three in Genomics: research in evolution of antibiotic resistance at Indiana Univ., \
             Georgetown Univ., and Purdue Univ.",
            "one in Synthetic Chemistry: Creation of novel macrocyclic compounds at Southwestern \
             Oklahoma State Univ.",
            "The Summer Science Program is an independent nonprofit, dedicated to creating a \
             transformational experience for our participants.  Many alumni call it “the \
             educational experience of a lifetime.”",
        ],
        identifier: "Summer Science Program",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(5.0),
        link: Some("https://summerscience.org/"),
        tuition: Some(9800.0),
        requirements: vec![
            Requirement::AgeRange(15..=18),
            Requirement::Custom(
                "While essay limits are 1000 characters, essays shorter than 500 characters are \
                 strongly recommended",
            ),
            Requirement::RequiredEssay(
                "What scholarly topic(s) do you currently find most interesting and why? When and \
                 how did you become aware of it? How have you explored it? Feel free to discuss a \
                 topic in STEM, out of STEM, or both.",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "We know you lead a busy life, full of activities, many of which are required of \
                 you. Tell us about something you choose to do for enjoyment or relaxation.",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "There are many ways to spend your summer. Why specifically are you applying to \
                 SSP, and how might you hope to have changed at the conclusion of your summer \
                 with us?",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "Communities can be large or small, well defined or informal. Tell us about a \
                 community you belong to and how engaging in that community has shaped your life \
                 and experiences.",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "Tell us about a non-academic challenge that you have faced, something in your \
                 family, your community, or your personal life that you are proud to have \
                 overcome and how you grew from the experience.",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "Reflect on a time you learned something from someone or a group of people who \
                 are unlike yourself and how that challenged your preconceptions or biases.",
                Limit::Characters(0..=1000),
            ),
            Requirement::RequiredEssay(
                "Is there a broader context that we should know about when comparing your \
                 qualifications (including grades and academics) to other applicants? Such as: 1. \
                 work after school/weekends (for pay or for a family business). 2. care for \
                 siblings or other relatives. 3. English as a second language. 4. relocated or \
                 changed schools. 5. health issues. 6. limited availability of advanced courses \
                 or STEM opportunities in your area. 7. other personal circumstances. For any \
                 'Yes', provide a brief description:",
                Limit::Characters(0..=500),
            ),
            Requirement::RequiredEssay(
                "Since starting high school, have you done any of the following? (If none of \
                 these apply to you, don't worry. We are just trying to get a fuller picture of \
                 how you spend your time.). 1. STEM-related internship or research project \
                 outside of school. 2. Another residential (live on a campus) science program. 3. \
                 Volunteer work. 4. Independent learning outside of schoolwork. For any \"Yes\", \
                 provide a brief description.",
                Limit::Characters(0..=500),
            ),
        ],
        specialization: Some(vec![
            (
                "Astrophysics",
                SummerCamp {
                    link: Some("https://summerscience.org/the-ssp-experience/orbit-determination/"),
                    ..Default::default()
                },
            ),
            (
                "Biochemistry",
                SummerCamp {
                    link: Some(
                        "https://summerscience.org/the-ssp-experience/fungal-inhibitor-design/",
                    ),
                    ..Default::default()
                },
            ),
            (
                "Genomics",
                SummerCamp {
                    link: Some("https://summerscience.org/the-ssp-experience/directed-evolution/"),
                    ..Default::default()
                },
            ),
            (
                "Synthetic Chemistry",
                SummerCamp {
                    link: Some("https://summerscience.org/synthetic-chemistry/"),
                    ..Default::default()
                },
            ),
        ]),
        ..Default::default()
    }
}
