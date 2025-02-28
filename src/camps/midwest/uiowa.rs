use crate::{
    camps::generators::make_specializations,
    structs::{Limit, Requirement, SummerCamp},
};

pub fn get_sstp() -> SummerCamp {
    SummerCamp {
        application_fee: Some(95.0),
        application_opens: Some("12/16/24"),
        deadline: Some("2/14/25"),
        description: vec![
            "If you are an advanced high school student in grades 10-11, this highly selective \
             and intensive summer research program may be for you. SSTP offers you rare access to \
             opportunities that help realize your academic and professional goals. You will \
             conduct research under the mentorship of world-class faculty from a \
             research-intensive university. Participate in classes and events that will stretch \
             you as a researcher and scholar. Explore your interests, enhance your academic \
             skills, and make meaningful friendships with intellectual peers.",
            "Research areas: Anatomy & Cell Biology, Anthropology, Biochemistry & Molecular \
             Biology, Biology, Biomedical Engineering, Business Analytics, Chemical & Biochemical \
             Engineering, Chemistry, Civil & Environmental Engineering, Genetics, Geographical & \
             Sustainability Sciences, Industrial & Systems Engineering, Internal Medicine, \
             Mechanical Engineering, Neurology, Neuroscience & Pharmacology, Obstetrics and \
             Gynecology, Orthopedics & Rehabilitation, Pathology, Psychiatry, Psychological & \
             Brain Sciences, and Religious Studies.",
        ],
        identifier: "UIowa Secondary Student Training Program",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(6.0),
        link: Some("https://belinblank.education.uiowa.edu/students/sstp/"),
        location: Some("Iowa City, Iowa"),
        organization: Some("Belin-Blank Center at University of Iowa"),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::RecommendationForm("academic reference (e.g. teacher)"),
            Requirement::RecommendationForm(
                "character reference (e.g. teacher, mentor, group leader)",
            ),
            Requirement::RequiredEssay(
                "Briefly describe each of your research areas of interest. This is used to help \
                 determine which faculty mentors will be sent your application materials. If you \
                 have particular interests, technical skills, or experience with a research \
                 method this is the appropriate place to highlight those talents.",
                Limit::Words(0..=750),
            ),
            Requirement::RequiredEssay(
                "Why are you applying to the Secondary Student Training Program? Describe your \
                 career aspirations and explain what you hope to gain from the SSTP experience.",
                Limit::Words(0..=750),
            ),
            Requirement::UnofficialTranscript,
            Requirement::TestScores("Enter scores that you feel best reflect your abilities"),
        ],
        tuition: Some(7500.0),
        notes: vec![],
        ..Default::default()
    }
}
pub fn get_young_writers() -> SummerCamp {
    SummerCamp {
        application_fee: Some(10.0),
        application_opens: Some("1/20/25"),
        deadline: Some("2/2/25"),
        description: vec![
            "The 2025 Summer Residential Program will have two sessions: June 15-28, 2025 \
             (Session 1) and July 13-26, 2025 (Session 2). The Summer Residential Program will \
             take place in Iowa City, Iowa on the University of Iowa campus. Students at the \
             program will spend 2 weeks deeply immersed in the art and craft of creative writing. \
             Each student will take one core course for the duration of the session. The core \
             courses are taught by graduates of the renowned Iowa Writers' Workshop and by \
             graduates of University of Iowa MFA programs in playwriting and nonfiction writing. \
             Students may select a core course in fiction writing, poetry writing, or creative \
             writing (which includes some combination of fiction, poetry, and personal essay). We \
             also offer core courses in TV writing and playwriting. Students will share their \
             writing with teachers and peers, receive constructive critique, and participate in \
             writing exercises and activities. To supplement the core courses, we'll also offer \
             readings by exciting published writers; workshops on process and aspects of craft; \
             discussions on writing-adjacent subjects (literary translation, revision); \
             collaborative projects to allow small groups of students to work together; as well \
             as open mics, talent shows, icebreakers, and social gatherings designed to give \
             students the opportunity to connect with one another. We believe that cultivating \
             and protecting one's writing life—the artistic practice, the inner journey of \
             exploration and discovery—should take precedence over pursuing a writing career, \
             though certainly the former can lead to the latter. Above all, we want to bring \
             adolescent writers together to celebrate all that they have in common, to encourage \
             them to explore themselves and their worlds, and to welcome them into the large \
             community of writers that radiates outward from Iowa City.",
        ],
        identifier: "UIowa Young Writers' Studio",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(2.0),
        link: Some("https://iyws.clas.uiowa.edu/2-week-summer-residential-program"),
        location: Some("Iowa City, Iowa"),
        organization: Some("University of Iowa"),
        requirements: vec![
            Requirement::GradeRange(11..=13),
            Requirement::RecommendationForm("English teacher"),
            Requirement::Custom("Writing sample, depending on track"),
            Requirement::RequiredEssay(
                "In 1-2 pages, tell us why you want to participate in the Studio and what you \
                 hope to accomplish here. You may want to discuss your writing sample or talk \
                 about writers or works that you admire. This is a chance for us to get to know \
                 you a little bit from your own words. The statement of purpose should be typed \
                 and double-spaced and should be submitted as either a Microsoft Word or PDF file.",
                Limit::Unspecified,
            ),
        ],
        tuition: Some(2500.0),
        notes: vec![],
        specialization: Some(make_specializations(&[
            "Poetry",
            "The Poetry course will explore voice, image, metaphor, line, language, and other \
             aspects of this most condensed form. If you love writing poetry and want to focus on \
             it exclusively and intensely, with a disciple's devotion, in a small tribe of fellow \
             poets, this is the course for you.",
            "Fiction",
            "If the short story or novel is your thing, then consider the Fiction course. By \
             focusing on challenges particular to fiction, such as establishing and deepening \
             characters, writing dialogue, using stream of consciousness and interior monologue, \
             working with 1st and 3rd and multiple points of view, managing plot and the passage \
             of time, and creating a vivid setting, this course will help you gain a deeper \
             understanding of the intricacies of the genre.",
            "Creative Writing",
            "Because of the scope of the class, which spans the three major genres of poetry, \
             fiction, and personal essay, the Creative Writing course concentrates on those \
             elements of writing that are universal: language, image, voice/point of view, \
             structure/story, setting, character. The Creative Writing course is a great choice \
             for you if your work already bridges multiple genres or if you want the freedom to \
             explore new forms.",
            "TV Writing/Writers' Room",
            "In this collaborative course, you'll work as a team with your fellow writers to \
             adapt a short story from the public domain into a dramatic television series. \
             Working with your instructor, who will act as the showrunner, you'll develop, \
             outline and write a pilot script. You'll learn about TV story structure, character \
             development, and conflicts, obstacles, dilemmas and goals. If you love dramatic \
             television, and want to write in the genre, this course is for you! Important: \
             Writers' Room is a collaborative course. You and your classmates will work together, \
             as a team, to develop and write a pilot script. That's why it's going to be so much \
             fun! While you may have the opportunity to roughly outline and pitch a personal \
             project, this will not be the focus of the course. ",
            "Playwriting",
            "In this core course, each student will focus on writing dialogue, developing \
             characters, building settings and scenes, and designing a plot with the aim of \
             finishing a one-act play by the end of the session. Students will read and discuss \
             their work in class and give each other constructive critique and ideas for \
             revision. Students will also read dramatic texts by established playwrights and \
             study them in terms of craft. If you've always wanted to write for the theatre, this \
             course is for you!",
        ])),
        ..Default::default()
    }
}
