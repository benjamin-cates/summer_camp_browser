use crate::structs::{Limit, Requirement, SummerCamp};

use super::super::generators::make_specializations;

pub fn get_all() -> impl Iterator<Item = SummerCamp> {
    let bu_aim = SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/academic-immersion/"),
        description: vec![
            "Here’s your chance to do a deep dive into a subject that fascinates you. Picture \
             this: three stimulating weeks on campus engaging in intensive study, with classmates \
             who feel just as passionate about a single academic topic as you do.",
        ],
        deadline: Some("Rolling ASAP"),
        identifier: "Boston University Academic Immersion (BU AIM)",
        length_wk: Some(3.0),
        requirements: vec![
            Requirement::UnofficialTranscript,
            Requirement::RecommendationForm("Any teacher"),
            Requirement::GradeRange(11..=12),
            Requirement::RequiredEssay(
                "Enter your personal statement describing why you want to attend the Academic \
                 Immersion: Introduction to Experimental Psychology, Introduction to Medicine, \
                 Creative Writing , or Business program below. Please provide examples of \
                 academic achievements, motivations, and goals that illustrate your interest in \
                 studying psychology, medicine, creative writing or business. Formatting from \
                 Word or any other word processor will be stripped out. (500 words to 3500 \
                 characters).",
                Limit::Characters(3000..=3500),
            ),
        ],
        application_fee: Some(60.0),
        tuition: Some(11000.0),
        apply_link: Some(
            "https://www.bu.edu/summer-registration/high-school-programs-academic-immersion/",
        ),
        last_updated: Some("1/8/25"),
        location: Some("Boston, Massachusetts"),
        organization: Some("Boston University"),
        specialization: Some(make_specializations(&[
            "Introduction to Experimental Psychology",
            "AIM: Introduction to Experimental Psychology combines classroom work in the \
             psychological sciences with the experience of conducting research. You will learn to \
             design and carry out your own research experiments under the supervision of Boston \
             University instructors.",
            "Introduction to Medicine",
            "AIM: Introduction to Medicine is a premedical program that combines classroom work \
             in the sciences with experiential learning activities that include simulations, lab \
             work, and explorations of different medical fields.",
            "Creative Writing",
            "AIM: Creative Writing gives you the opportunity to develop your writer’s voice in a \
             supportive, inspiring environment. You will learn formal techniques in multiple \
             genres from seasoned instructors and participate in workshops that build confidence \
             and sharpen skills. You will hear published authors discuss their craft, engage in \
             free-write exercises, and visit important cultural centers around Boston. This \
             exciting track is offered in partnership with BU’s renowned Creative Writing \
             Program. Writing sample: Creative Writing applicants will need to submit a short (< \
             500 words) sample of their original creative writing.",
            "Business",
            "AIM: Business is offered in partnership with BU’s Questrom School of Business, a top \
             20 undergraduate business program. This track combines interactive lectures, case \
             studies, and local field trips. You will work in teams to design and carry out your \
             own New Product Development (NPD) Business Plan. At the end of the program, each \
             team will present their work to their peers as well as selected BU Questrom School \
             of Business faculty.",
        ])),
        keywords: vec![("psychology", 10), ("business", 10), ("writing", 10), ("medicine", 10)],
        ..Default::default()
    };
    let bu_hsh = SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/high-school-honors/"),
        description: vec![
            "Ready to push your limits academically? Excited to get out of your comfort zone, \
             explore new subjects, and earn college credit? If this sounds like you—and you are \
             entering your junior or senior year—High School Honors offers a Boston University \
             pre-college experience that will challenge you and expand your horizons. Select two \
             BU undergraduate courses from more than 80 available offerings in a variety of \
             subjects.",
        ],
        deadline: Some("5/23/25"),
        identifier: "Boston University High School Honors (BU HSH)",
        length_wk: Some(6.0),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::RecommendationForm("High School Counselor"),
            Requirement::RecommendationForm("Teacher"),
            Requirement::RequiredEssay(
                "Describe why you want to attend the High School Honors program.",
                Limit::Words(500..=750),
            ),
            Requirement::UnofficialTranscript,
            Requirement::TestScores("Optional SAT, ACT, or AP"),
            Requirement::Custom("Average unweighted GPA acceptance was 3.76"),
        ],
        application_fee: Some(60.0),
        tuition: Some(12000.0),
        apply_link: Some(
            "https://www.bu.edu/summer-registration/high-school-programs-high-school-honors/",
        ),
        last_updated: Some("1/9/25"),
        location: Some("Boston, Massachusetts"),
        organization: Some("Boston University"),
        ..Default::default()
    };
    let bu_rise = SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/rise-internship-practicum/"),
        description: vec![
            "If you’re passionate about the sciences and are a domestic student currently in your \
             junior year of high school, we invite you to apply for the Research in Science & \
             Engineering (RISE) program. You will spend six weeks at BU conducting university \
             laboratory research with some of the nation’s brightest scientific minds while \
             advancing your STEM knowledge and skills.",
        ],
        deadline: Some("2/14/25"),
        identifier: "Boston University RISE",
        length_wk: Some(6.0),
        requirements: vec![
            Requirement::GradeRange(12..=12),
            Requirement::RequiredEssay(
                "Why you selected your subject of interest",
                Limit::Words(0..=300),
            ),
            Requirement::RequiredEssay("Your academic achievements", Limit::Words(0..=250)),
            Requirement::RequiredEssay(
                "Why you want to attend the RISE program",
                Limit::Words(0..=200),
            ),
            Requirement::RecommendationForm(
                "Science teacher, math teacher, or research advisor, due 2/21/25",
            ),
            Requirement::UnofficialTranscript,
            Requirement::TestScores("Optional SAT, ACT, or AP"),
        ],
        application_fee: Some(60.0),
        tuition: Some(11000.0),
        apply_link: Some("https://www.bu.edu/summer-registration/high-school-programs-research/"),
        last_updated: Some("1/9/25"),
        location: Some("Boston, Massachusetts"),
        organization: Some("Boston University"),
        specialization: Some(make_specializations(&[
            "Internship",
            "You will tackle research projects under the mentorship of distinguished faculty, \
             postdoctoral fellows, and graduate students. Opt for the Internship track and you \
             will spend 40 hours each week working on research projects designed by your mentor \
             to help you acquire valuable technical and analytical skills while developing \
             insight into the scientific process. You may choose from hands-on research \
             opportunities in areas such as: astronomy, biology, biomedical engineering, \
             chemistry, computer science, electrical and computer engineering, mechanical \
             engineering, medical laboratory research, neuroscience, physics, psychology, and \
             public health. You will present the results of your research at the Poster Symposium \
             that concludes the program. In addition, one day per week you will join the \
             Practicum students for workshops aimed at building your academic and professional \
             skills.",
            "Practicum",
            "In summer 2025, you can choose to study either Computational Neurobiology or Data \
             Science. You will conduct group research in a university setting under the guidance \
             of a BU instructor. Ideal for students who prefer a structured research environment. \
             Opt for the Practicum track and you will begin your days with a two-hour lecture \
             from your instructor before continuing in the afternoon with four hours of group \
             research. The experiments are designed to demonstrate procedures that are \
             representative of university laboratory research. Although this course has a set \
             syllabus with clearly outlined protocols, the final outcome of your computational \
             research project cannot be predicted in advance. You will present the results of \
             your research project alongside RISE Internship students at the Poster Symposium \
             that concludes the program.",
        ])),
        keywords: vec![("internship", 10), ("practicum", 10), ("data science", 10), ("neurobiology", 10)],
        ..Default::default()
    };
    let bu_summer_challenge = SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/summer-challenge/"),
        description: vec![
            "It’s called Summer Challenge because it pushes you in so many ways. You will explore \
             your interests, discover new ones, test your academic boundaries, and immerse \
             yourself in the college experience. Along the way, you will make new friends and get \
             to know more about Boston University. Are you up to the challenge?",
        ],
        deadline: Some("Rolling ASAP"),
        identifier: "Boston University Summer Challenge",
        length_wk: Some(2.0),
        requirements: vec![
            Requirement::GradeRange(10..=12),
            Requirement::RequiredEssay(
                "Describe why you want to attend the Summer Challenge program.",
                Limit::Words(500..=750),
            ),
            Requirement::UnofficialTranscript,
        ],
        application_fee: Some(60.0),
        tuition: Some(9000.0),
        apply_link: Some(
            "https://www.bu.edu/summer-registration/high-school-programs-summer-challenge/",
        ),
        last_updated: Some("1/9/25"),
        location: Some("Boston, Massachusetts"),
        organization: Some("Boston University"),
        ..Default::default()
    };
    let bu_summer_preview = SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/summer-preview/"),
        description: vec![
            "Get a taste of college life for one exciting week this summer. Delve into \
             fascinating subjects. Make new friends. Explore the fun, vibrant city of Boston. Our \
             Summer Preview program offers rising eighth, ninth, and tenth graders a choice of \
             three different weeklong, noncredit seminars. It’s a great opportunity for you to \
             dig into an area of academic interest or become familiar with an entirely new \
             subject.",
        ],
        deadline: Some("Rolling ASAP"),
        identifier: "Boston University Summer Preview",
        length_wk: Some(1.0),
        requirements: vec![
            Requirement::GradeRange(8..=10),
            Requirement::RequiredEssay(
                "Describe why you want to attend the Summer Preview program.",
                Limit::Words(500..=750),
            ),
            Requirement::UnofficialTranscript,
            Requirement::Custom("Transcript must include the last two years of school"),
        ],
        application_fee: Some(60.0),
        tuition: Some(4000.0),
        apply_link: Some(
            "https://www.bu.edu/summer-registration/high-school-programs-summer-preview/",
        ),
        last_updated: Some("1/9/25"),
        location: Some("Boston, Massachusetts"),
        organization: Some("Boston University"),
        ..Default::default()
    };

    [
        bu_rise,
        bu_aim,
        bu_hsh,
        bu_summer_challenge,
        bu_summer_preview,
    ]
    .into_iter()
}
