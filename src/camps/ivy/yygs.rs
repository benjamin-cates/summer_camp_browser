use crate::{camps::generators::make_specializations, structs::{Limit, Requirement, SummerCamp}};


pub fn get_yygs() -> SummerCamp {
    SummerCamp {
        link: Some("https://globalscholars.yale.edu/"),
        application_fee: Some(75.0),
        #[rustfmt::skip]
        apply_link: Some("https://apply.globalscholars.yale.edu/apply/?sr=b1600633-938a-43f0-950d-f31d81fc37ed"),
        deadline: Some("10/20/24"),
        description: vec![
            "Yale Young Global Scholars (YYGS) is one of the most globally diverse, two-week \
             academic summer programs in the world. Serving over 2,000 students from 150+ \
             countries and all 50 U.S. states, YYGS distributes over $3 Million USD in need-based \
             financial aid to both domestic and international students. Our curriculum is highly \
             collaborative, exposing participants to innovative topics while facilitating student \
             discussions shaped by international perspectives. Students have the opportunity to \
             participate in a STEM, social sciences, humanities, or cross-disciplinary track.",
        ],
        last_updated: Some("1/7/25"),
        identifier: "Yale Young Global Scholars (YYGS)",
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::AgeRange(16..=18),
            Requirement::LetterOfRec("anyone"),
            Requirement::OfficialTranscript,
            Requirement::RequiredEssay(
                "How many session preferences would you like to indicate? How many session \
                 preferences would you like to indicate?  ❑ 1 - Please consider me for my first \
                 choice session ONLY ❑ 2 - Please consider me for my first and second choice \
                 sessions ONLY ❑ 3 - Please consider me for my first, second, and third choice \
                 sessions ONLY ❑ 4 - Please consider me for my first, second, third, and fourth \
                 choice sessions. Please explain why you chose the session(s) above",
                Limit::Words(0..=100),
            ),
            Requirement::RequiredEssay(
                "We know students have many options for academic enrichment programs to which to \
                 apply. Please explain why you chose to apply for YYGS specifically AND why you \
                 would be a great fit for the program. What will you contribute to YYGS, and what \
                 do you hope to take away from YYGS and bring back to impact your community?",
                Limit::Words(0..=100),
            ),
            Requirement::RequiredEssay(
                "Which dates are you AVAILABLE to attend YYGS? Please select all that apply. \
                 Which session dates would be your FIRST CHOICE preference to attend YYGS? Please \
                 select only one. Please explain your responses above, including both your \
                 availability across all three session dates as well as your first choice session \
                 dates.",
                Limit::Words(0..=100),
            ),
            Requirement::RequiredEssay(
                "Please reflect on a time when you questioned or challenged a belief or idea. \
                 What prompted your thinking? What was the outcome?",
                Limit::Words(200..=400),
            ),
            Requirement::RequiredEssay(
                "Fast take: If someone offered you a box with every inanimate object you have \
                 ever lost, what is the first thing you are looking for? Why?",
                Limit::Characters(0..=280),
            ),
            Requirement::RequiredEssay(
                "Fast take: What's one skill, talent, or hobby that you have always wanted to \
                 learn how to do?",
                Limit::Characters(0..=280),
            ),
            Requirement::RequiredEssay(
                "We want to learn more about your background, beliefs, values, and/or the \
                 important people in your life. Please tell us about something that has \
                 influenced you and articulate how it has shaped you.",
                Limit::Words(0..=200),
            ),
            Requirement::ActivityList(
                "List a maximum of five (5) previous awards, honors, scholarships, fellowships, \
                 and/or recognitions along with the date(s) they were received. Enter \"Activity \
                 name/Organization\": 66 chars, \"Position\": 65 chars, \"Description\": 130 \
                 chars, \"Why is this activity most meaningful to you?\": 100 words",
            ),
        ],
        location: Some("New Haven, Conneticut"),
        organization: Some("Yale Young Global Scholars"),
        tuition: Some(6500.0),
        length_wk: Some(2.0),
        specialization: Some(make_specializations(&[
            "Innovations in Science & Technology (IST)",
            "Innovations in Science & Technology (IST) is designed for students who are \
             interested in learning about diverse topics in the STEM fields and applying \
             scientific principles to real-world applications. Students explore a wide variety of \
             scientific fields such as physics, molecular biology, chemistry, biochemistry, \
             astronomy, engineering, neuroscience, immunology, psychology, and earth science. \
             Students also are exposed to interdisciplinary applications across the many \
             scientific fields ranging from the nanoscopic to the astronomical in scale.",
            "Literature, Philosophy, & Culture (LPC)",
            "Literature, Philosophy, & Culture (LPC) is designed for students with an interest in \
             the expression and interpretation of creativity and culturally significant texts. \
             Participants study fiction, philosophy, poetry, theater, film, music, visual arts, \
             dance, and other creative arts.",
            "Politics, Law, & Economics (PLE)",
            "Politics, Law, & Economics (PLE) is a session aimed at students with an interest in \
             understanding diverse economic theories, the values and practices of government, and \
             legal frameworks in historical and comparative perspectives. Students learn key \
             ideas in topics such as public policy, human rights, market regulation, governance \
             structures, international policy, and conflict and cooperation across borders.",
            "Solving Global Challenges (SGC)",
            "Solving Global Challenges (SGC) is designed to equip students with innovative and \
             cross-disciplinary strategies to address the most pressing global issues of the 21st \
             century. Guided by the United Nations’ 17 Sustainable Development Goals (link is \
             external), the program covers a broad range of critical topics, including global \
             health, gender equality, poverty alleviation, to environmental sustainability. SGC \
             provides students with the tools to translate technical knowledge into effective \
             solutions. By integrating diverse academic disciplines—from STEM and social sciences \
             to the humanities and arts—students develop and refine their skills in critical \
             analysis, problem-solving, and creativity as they work towards sustainable solutions \
             to the complex issues that will shape our future.",
        ])),
        acceptance_rate: Some(15.0),
        ..Default::default()
    }
}
