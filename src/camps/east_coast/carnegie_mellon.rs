use crate::structs::{Limit, Requirement, SummerCamp};

pub fn get() -> SummerCamp {
    SummerCamp {
        deadline: Some("2/1/25"),
        description: vec![],
        identifier: "Carnegie Mellon Precollege Programs",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        link: Some("https://www.cmu.edu/pre-college/academic-programs/index.html"),
        location: Some("Pittsburgh, Pennsylvania"),
        organization: Some("Carnegie Mellon University"),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::AgeRange(16..=18),
            Requirement::Custom("additional optional supplemental materials"),
            Requirement::UnofficialTranscript,
            Requirement::TestScores("optional"),
        ],
        specialization: Some(vec![
            (
                "AI Scholars",
                SummerCamp {
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("prefer STEM teacher"),
                        Requirement::RecommendationForm("counselor, advisor, or leader"),
                    ],
                    description: vec![
                        "AI Scholars will take a deep dive into the world of artificial \
                         intelligence through a combination of classroom instruction, hands-on \
                         research projects, faculty lectures, and industry engagement with \
                         leading tech companies around the country.",
                    ],
                    length_wk: Some(4.0),
                    application_fee: Some(0.0),
                    tuition: Some(0.0),
                    ..Default::default()
                },
            ),
            (
                "Architecture",
                SummerCamp {
                    description: vec![
                        "In these changing times, Pre-College Architecture responds to challenges \
                         regarding practice, communication, and our relationship with the built \
                         environment by utilizing a diverse set of tools and techniques, guided \
                         by the leadership of Carnegie Mellon Architecture Faculty. Pre-College \
                         Architecture introduces design practice, creative problem solving, and \
                         critical thinking to young designers through a matrix of programming, \
                         celebrating curiosity and providing a strong foundation for \
                         architectural education. Pre-College students will navigate complex \
                         design problems, encouraging growth through the development of skills \
                         and preparing them for future undergraduate studies.",
                    ],
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Why are you interested in studying architecture?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "30 second video Identify and describe something about yourself that \
                             will help the admission committee determine your potential as a \
                             Pre-College Architecture student at Carnegie Mellon University. We \
                             are interested in getting to know you in the context of your \
                             application.  This short 30-second video needs no staging or \
                             post-production.",
                            Limit::Unspecified,
                        ),
                        Requirement::RequiredEssay(
                            "5 to 10 images of your original art work in a single PDF file",
                            Limit::Unspecified,
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    length_wk: Some(5.0),
                    application_fee: Some(50.0),
                    tuition: Some(10381.0),
                    ..Default::default()
                },
            ),
            (
                "Art",
                SummerCamp {
                    application_fee: Some(50.0),
                    tuition: Some(11791.0),
                    description: vec![
                        "In this program, you will use both traditional tools and cutting-edge \
                         technologies to explore your creative ideas in a college-level studio \
                         environment. You will develop both your conceptual and technical skills, \
                         preparing you for a wide range of opportunities in both art studies and \
                         careers. You will break through the boundaries of how you approach art, \
                         taking your work to the next level.",
                    ],
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Why do you want to attend CMU's Pre-College Art Program?",
                            Limit::Words(150..=200),
                        ),
                        Requirement::RequiredEssay(
                            "What resources do you wish were available in your area to make you a \
                             better artist?",
                            Limit::Words(150..=200),
                        ),
                        Requirement::RequiredEssay(
                            "What artwork do you make outside of assigned school projects, and \
                             what excites you about this process of creating?",
                            Limit::Words(150..=200),
                        ),
                        Requirement::RequiredEssay(
                            "5 to 10 images of your original art work in a single PDF file",
                            Limit::Unspecified,
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    notes: vec!["This program is offered in 3 or 6 week format"],
                    ..Default::default()
                },
            ),
            (
                "CS Scholars",
                SummerCamp {
                    description: vec![
                        "CS Scholars (CSS), a CS Pathways initiative, is a four-week immersive \
                         experience on the CMU campus for rising high school juniors. This fully \
                         funded, merit-based program brings students together from around the \
                         country to explore computer science with Carnegie Mellon faculty, staff, \
                         and researchers who are leaders in the field. As a fully funded program, \
                         CS Scholars program costs are covered by CS Pathways through generous \
                         support from foundations, corporations, sponsors, and donors. Covered \
                         costs include tuition, housing, meal plans, and program related field \
                         trips and special events. Travel assistance may be requested by families \
                         for whom transportation costs to and from CMU would provide a \
                         significant barrier to attendance. As a CS Scholar, you will engage in \
                         college-level courses, as well as weekly college prep and readiness \
                         seminars focused on college admissions, financial aid, social emotional \
                         wellbeing, and more. You'll develop meaningful connections with peers \
                         and mentors while being fully immersed in the college experience. A core \
                         value of CS Pathways and the CS Scholars program is building an \
                         inclusive, anti-racist environment. You will be expected to uphold this \
                         value and to participate in discussions about equity within STEM and \
                         beyond.",
                    ],
                    length_wk: Some(4.0),
                    application_fee: Some(0.0),
                    tuition: Some(0.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("prefer STEM teacher"),
                        Requirement::RecommendationForm("counselor, advisor, or leader"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Computational Biology",
                SummerCamp {
                    description: vec![
                        "The Pre-College Program in Computational Biology offers comprehensive \
                         training in both state-of-the-art laboratory techniques for generating \
                         biological data and the computational methods necessary to analyze it. \
                         Computer science has transformed biology and medicine. The next \
                         generation of life scientists must be well-versed not only in laboratory \
                         techniques for data generation but also in the computational skills \
                         needed to analyze and model this data. Pre-College Computational Biology \
                         provides high school students with an exceptional opportunity to explore \
                         this interdisciplinary relationship in a university setting.",
                    ],
                    length_wk: Some(4.0),
                    application_fee: Some(50.0),
                    tuition: Some(10000.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Why are you interested in studying Computational Biology?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Design",
                SummerCamp {
                    description: vec![
                        "Design has never been in greater demand than it is now. Businesses, \
                         government, and local communities are turning to designers for \
                         innovation and problem-solving, creating new roles for designers in \
                         every sector of the marketplace. Pre-College Design introduces students \
                         to the discipline of design, provides a foundation of skills, and gives \
                         a clear idea of what to expect from a college-level accredited program. \
                         Pre-College Design students will have the opportunity to attend classes \
                         taught by full-time Carnegie Mellon faculty where they are given an \
                         introduction to the design principles and practices that are the \
                         cornerstones of the undergraduate School of Design program.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(50.0),
                    tuition: Some(11791.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Why are you interested in studying Design?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("anyone"),
                        Requirement::Custom("Portfolio"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Drama",
                SummerCamp {
                    description: vec![
                        "Pre-College Drama prepares students for the college audition and \
                         interview process, illustrating the kind of creativity and discipline \
                         required of students studying a Bachelor of Fine Arts Drama curriculum. \
                         Classes are taught by accomplished Carnegie Mellon University faculty \
                         and adjunct professors.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(50.0),
                    tuition: Some(11887.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Describe a non-theater related life experience where your theater \
                             background helped you solve a problem or create a positive change.",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "This program is six weeks in length and involves rigorous training \
                             and a commitment to reliable attendance and solid work. What \
                             practical skills and habits have you developed to show that you are \
                             prepared to commit fully to the program?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Music - 6 Week",
                SummerCamp {
                    description: vec![
                        "Pre-College Music mirrors the first weeks of Carnegie Mellon’s \
                         first-year undergraduate School of Music curriculum. It is designed to \
                         prepare students for college-level auditions and the life of a music \
                         major. Students build skills and confidence through private studio \
                         lessons, major performing ensembles, and music support courses. \
                         Admission to each course is based on a demonstrated performance level, \
                         which can be shown through an audition recording or a submitted \
                         portfolio. Regardless of a student’s skill level, there is a place for \
                         them to learn and grow in this program. Our residential 6-week program \
                         stands out as the most comprehensive and enriching option, yielding the \
                         most significant results. This immersive experience allows students to \
                         explore the full breadth of our curriculum, advancing to more advanced \
                         repertoires both in solo work and ensemble performances.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(50.0),
                    tuition: Some(11246.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("anyone"),
                        Requirement::Custom("Portfolio depending on track"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "National High School Game Academy",
                SummerCamp {
                    description: vec![
                        "The National High School Game Academy (NHSGA) allows disciplined, \
                         passionate, and dedicated high school students to experience video game \
                         development using current industry best practices. Sharing methods and \
                         paradigms used in Carnegie Mellon's Masters of Entertainment Technology \
                         (M.E.T.) graduate program, the NHSGA focuses on an innovative blend of \
                         hands-on team projects combined with in-depth traditional classwork, \
                         lecture and discussion. As a verification of competence, homework is \
                         assigned and evaluated as the student progresses through the curriculum. \
                         Designed to elevate a student’s rigor, working process, and discipline, \
                         the NHSGA prepares students for their college education. As a \
                         culmination of the six weeks, student teams create an original game \
                         prototype from pitch to final playable demo.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(50.0),
                    tuition: Some(12529.0),
                    requirements: vec![
                        Requirement::Custom(
                            "Examples of applicant work relevant to their specific track(s) of \
                             interest (i.e., Software Development, Visual Arts/Graphic Design, \
                             Sound Design or Music Composition, Game Generalist)",
                        ),
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Tell us about your favorite game. Why do you find it fun? Be \
                             specific about the aspects that make it fun for you. Lastly, what \
                             emotion did you feel while playing the game. Why do think this \
                             occurred?",
                            Limit::Words(0..=300),
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Summer Academy for Math and Science",
                SummerCamp {
                    description: vec![
                        "For over twenty years, the Summer Academy for Math and Science (SAMS) \
                         program has engaged students in a rigorous curriculum taught by faculty \
                         and staff who are deeply committed to student success. Led by the Center \
                         for Student Diversity & Inclusion, SAMS allows students to develop a \
                         deeper understanding of STEM via traditional classroom instruction, \
                         through hands-on projects, and sustained engagement with world-renowned \
                         faculty and skilled staff mentors. In addition to their academic \
                         experiences, students also have the opportunity to collaborate and \
                         develop meaningful relationships with peers from across the country.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(0.0),
                    tuition: Some(0.0),
                    requirements: vec![
                        Requirement::GradeRange(12..=12),
                        Requirement::RecommendationForm("current math teacher"),
                        Requirement::RecommendationForm("anyone"),
                        Requirement::RequiredEssay(
                            "Respond to one of these prompts: 1. Carnegie Mellon’s founder, \
                             Andrew Carnegie, is famously known for the quote “My Heart is in the \
                             Work.” At the Center for Student Diversity & Inclusion, our heart is \
                             in work which focuses on diversity, equity, inclusion and belonging \
                             (DEIB). How will being a part of the SAMS cohort help you amplify or \
                             discover your STEM-inspired passions? 2. As a STEM scholar, how can \
                             your academic work address a social inequity you have personally \
                             experienced or observed as harming/impacting a community in which \
                             you belong?",
                            Limit::Words(0..=1000),
                        ),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Summer Session",
                SummerCamp {
                    description: vec![
                        "Pre-College Summer Session provides high school students with the unique \
                         opportunity to take summer courses offered by Carnegie Mellon University \
                         for credit. Students earn college credit while working in an academic \
                         setting that mirrors the supportive, rigorous environment of the first \
                         year of college. Every year, Summer Session offers approximately thirty \
                         courses from academic disciplines across the university, including: \
                         science, humanities, social science, engineering, computer science, and \
                         technology applied to the arts. Successful Summer Session students \
                         leverage their experiences to demonstrate their ability to succeed in \
                         college. Upon successful completion of a course, they receive college \
                         credit at Carnegie Mellon University, which is transferable to other \
                         institutions that accept CMU college courses for credit.",
                    ],
                    length_wk: Some(6.0),
                    application_fee: Some(50.0),
                    tuition: Some(13792.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "What has prepared you for the challenge of rigorous college-level \
                             courses?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    ..Default::default()
                },
            ),
            (
                "Writing & Culture",
                SummerCamp {
                    description: vec![
                        "The idea of the attic-bound creative is a myth. Regardless of the \
                         medium, filmmakers and writers live their lives out in the world, not \
                         hidden behind closed doors. In this program, students explore writing, \
                         film, and culture through traditional, historical, and contemporary \
                         lenses on CMU’s campus and throughout Pittsburgh. Over the weeks, \
                         students will produce a body of work anchored in digital poetics and \
                         short-form writing, film development and production, and preparatory \
                         materials setting a strong foundation in the humanities. But this is not \
                         a Language Arts or Film Production course. By participating in this \
                         program, students will learn to think critically, express thoughts \
                         creatively, and communicate effectively for college and beyond. They \
                         will build a diverse body of work, moving through a series of written \
                         and visual formats and culminating in a short film. In the course of \
                         developing their stories, students will explore the continuum of both \
                         written and visual communication.",
                    ],
                    length_wk: Some(4.0),
                    application_fee: Some(50.0),
                    tuition: Some(9310.0),
                    requirements: vec![
                        Requirement::RequiredEssay(
                            "What do you hope to gain from participating in Carnegie Mellon's \
                             Pre-College Programs?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "What kinds of media (film, books, etc.) do you enjoy most and why?",
                            Limit::Words(300..=500),
                        ),
                        Requirement::RequiredEssay(
                            "Tell us about what kinds of work you have produced, whether in \
                             writing or across various kinds of media.",
                            Limit::Words(300..=500),
                        ),
                        Requirement::Custom(
                            "Portfolio of selected work that can include at least three samples, \
                             such as: videos, written works, or other artistic creations that are \
                             relevant to the program",
                        ),
                        Requirement::RecommendationForm("anyone"),
                    ],
                    ..Default::default()
                },
            ),
        ]),
        notes: vec![],
        ..Default::default()
    }
}
