use crate::{
    camps::generators::make_specializations,
    structs::{Limit, Requirement, SummerCamp},
};

// 2/19/2008
pub fn get_humanities() -> SummerCamp {
    SummerCamp {
        application_fee: Some(65.0),
        apply_link: Some(
            "https://spcsonlineapp.stanford.edu/register/?id=30dddc68-c0c3-40d2-b0fe-679e03866682",
        ),
        deadline: Some("2/3/25"),
        description: vec![
            "Not for credit or grade, Stanford Summer Humanities Institute encourages participants to tap into Stanford University's excellence in the humanities and social sciences—exploring texts and ideas at a profound level, writing college-level papers, and communicating complex arguments in their academic writing and discussion sections.",
            "Intensive Exploration: Participants investigate topics in the humanities that teach them to understand the past, question the present, and imagine the future.",
            "Profound Conversations: From breakfast to evening activities, participants have the chance to discuss powerful ideas introduced in lectures, readings, and discussion sections.",
            "Global Citizenship: Participants from around the world come together as a community of curious, independent thinkers from a wide range of backgrounds and perspectives.",
            "Independent Research: During their third week, participants work closely with Stanford professors, graduate students, and writing mentors to produce original research projects.",
            "Courses meet each weekday from 9:00am to 11:00am Pacific Time for faculty-led sessions that include lecture, small group discussions, and individual work. Discussion sections, led by Stanford graduate students, meet each weekday from 1:00pm to 2:00pm Pacific Time in smaller groups to dive deeper into topics covered in faculty-led sessions.",
        ],
        identifier: "Stanford Summer Humanities Institute (SHI)",
        keywords: vec![],
        last_updated: Some("1/13/25"),
        length_wk: Some(3.0),
        link: Some("https://summerhumanities.spcs.stanford.edu/about-humanities"),
        location: Some("Palo Alto, California"),
        organization: Some("Stanford University"),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::AgeRange(16..=18),
            Requirement::RecommendationForm("teacher of English, history, or social science."),
            Requirement::Custom("Select four prefered courses in order"),
            Requirement::Custom(
                "We require unofficial transcripts for each school or academic program from which \
                 you earned grades (such as letter or numerical grades) from Fall 2022 through \
                 Fall 2024. If you have multiple grade reports from one school, rather than one \
                 transcript from that school, combine that into a single file for upload.",
            ),
            Requirement::Custom(
                "One sample of student work in the humanities or social sciences will be \
                 required. The work sample must have been completed as a school assignment within \
                 the past year and must demonstrate a student’s best effort. Though not required, \
                 your sample should show graded work and the prompt or directions. No work sample \
                 older than two years will be accepted. Suggested samples include: analytical \
                 essays, persuasive essays, or research papers. Limit the sample to five pages of \
                 content in addition to any directions or grading sheets.",
            ),
            Requirement::Custom(
                "For the work sample: briefly describe the written work sample you are submitting.",
            ),
            Requirement::Custom("For the work sample: why did you select this work sample?"),
            Requirement::RequiredEssay(
                "Describe the kind of student you are. What strengths are you proud of? For \
                 example: curiosity, classroom engagement, or dedication. Include specific \
                 examples from academic experiences.",
                Limit::Words(50..=250),
            ),
            Requirement::RequiredEssay(
                "What impact would you like to have on your community, or on society in general, \
                 in the next five to ten years?",
                Limit::Words(50..=200),
            ),
            Requirement::RequiredEssay(
                "Explain what interests you about the humanities and about each of the SHI \
                 course(s) you are applying to. Provide examples of specific experiences or \
                 topics you have learned about that incited you to further explore the humanities.",
                Limit::Words(100..=400),
            ),
            Requirement::RequiredEssay(
                "What are your main reasons for applying to Stanford Summer Humanities Institute?",
                Limit::Words(50..=200),
            ),
            Requirement::RequiredEssay(
                "Have you participated in a Stanford Pre-Collegiate Studies program before, such \
                 as: Stanford Math Circle, Stanford Pre-Collegiate Summer Institutes (SI), \
                 Stanford University Mathematics Camp (SUMaC), Stanford Middle School Scholars \
                 Program (SMSSP), Stanford Summer Humanities Institute (SHI), or Stanford \
                 University-Level Online Math & Physics (ULO)? Have you enrolled in a course at \
                 Stanford Online High School (OHS)? If yes, name the program(s) and your dates of \
                 attendance below. If no, enter N/A in the box below.",
                Limit::Words(0..=100),
            ),
            Requirement::OptionalEssay(
                "List all other summer programs you are applying to for this upcoming summer.",
                Limit::Words(0..=100),
            ),
            Requirement::Custom(
                "This video essay is an opportunity to share more about your interest in our \
                 program. You can consider it a conversation between you and our Admissions \
                 Committee. You must record the two minute video essay in the web page in any \
                 number of takes. Answer the question: \"Tell us two things that compelled you to \
                 apply to a Stanford Pre-Collegiate Studies summer program.\"",
            ),
            Requirement::TestScores("Optional, ACT, PSAT, SAT, AP Exams"),
        ],
        tuition: Some(8575.0),
        specialization: Some(make_specializations(&[
            "Ancient Rome and Its Legacies",
            "From a few huts on seven hills in the eighth century BCE (or so they say), Rome \
             became an empire that, at its greatest expanse, encompassed western Europe, North \
             Africa, the Middle East, and lands surrounding the Black Sea. More impressive than \
             its expanse, its eastern half would last until 1453. Rome’s influence remains \
             visible today: its roads, many still in use, formed a network along which goods and \
             ideas traveled; its awe-inspiring architecture, from the celestial Pantheon to the \
             magnificent Pont Du Gard, set standards not to be reached again for over a \
             millennium; its law code provides the foundation for Civil Law, and its art, like \
             the murals in Nero’s Domus Aurea, and its literature, including Vergil’s Aeneid and \
             Tacitus’ Annals, continues to inspire. In this course we will study aspects of this \
             historical phenomenon: read many of its most famous works, reflect on how the Romans \
             thought of themselves and others, and trace the history of one of its texts, \
             considered most dangerous by some. We will follow Rome’s rise and fall as an empire, \
             and remark throughout on how different it is from Western societies today, even \
             though the latter are profoundly indebted to it.",
            "The American Enlightenment",
            "The American Enlightenment spanned the years roughly 1770 to 1820, some of the most \
             exciting and tumultuous in American and European history. During this half century, \
             such world-changing events as the American, Haitian, and French Revolutions, and the \
             transatlantic Enlightenment stretched people's thinking into many new and unexpected \
             directions. This course will examine a few of these new ideas. We will study how \
             America became a laboratory for exciting new ideas about politics, nature, rights, \
             and humanity. The course will explore these ideas through original writings from \
             figures such as Thomas Jefferson, and famous contemporaries such as Voltaire and \
             Rousseau. Our discussions will be framed around a series of questions that noted \
             thinkers of the American Enlightenment fiercely debated and that continue to inspire \
             us today.",
            "Books to Bollywood",
            "A good story rarely stays put for long. Since the advent of Indian cinema more than \
             a century ago, novels, poetry, and short stories have provided a rich source of \
             inspiration for filmmakers. As these stories move across time, space, and media \
             forms, their meanings shift, sparking new interpretations and cultural \
             conversations. In this course, we will study films alongside their literary \
             inspirations, ranging from popular adaptations of the British canon to art film \
             interpretations of Urdu feminist literature. We will begin by examining the \
             reevaluation of the British canon, exploring how recent adaptations of Shakespeare \
             engage with the legacies of colonial education in India. We will then analyze \
             allegorical stories about the transition from Muslim to British sovereignty in South \
             Asia in the mid-19th century—stories that resonated with both British imperialists \
             and anti-colonial activists, and that found renewed popularity in 1970s and 1980s \
             cinema. Moving forward, we examine the promises and anxieties of decolonization and \
             the early independence period through stories about womens’ roles in society. \
             Finally, we will consider how literature and film have been perceived as threats and \
             explore the legal structures that have been used to suppress the arts. Along the \
             way, we will ask key questions: How does adaptation create meaningful differences? \
             What are the risks and opportunities of transforming a short story into a two-hour \
             epic? What are the power dynamics and potential aesthetic questions involved in \
             shifting language of production from English to Hindi? Does the process of \
             adaptation automatically uphold the values of the original, or can a story be made \
             to speak against itself? Is a good literary source necessary to produce a great \
             film? Students will gain familiarity with key terminology and analytical frameworks, \
             equipping them to write thoughtfully about different media forms. This course \
             invites students to examine the dynamic interplay between literature and film while \
             reflecting on the cultural and historical forces that shape their adaptations.",
            "Colonial Extractions of African Cultural Treasures",
            "The colonial era saw widespread extraction of cultural treasures by European powers \
             across the globe. Stanford University, for example, has a large collection of \
             African objects in the Cantor Museum, while in nearby San Francisco, the renowned De \
             Young Museum has a significant selection in its Africa gallery. Greece, Egypt, and \
             other countries have maintained that they belong at home rather than in the museums \
             of London, Paris, and New York. In this course we will consider the role of African \
             art in debates about ownership, access, and aesthetics. Lectures will chart the \
             “scramble for art” that occurred in the nineteenth and early twentieth centuries \
             among European colonial powers on the African continent. We will also examine the \
             role of North American collectors in extracting African cultural treasures, and the \
             burgeoning ethnographic museum culture that showcased these objects at universities \
             and museums across the United States. We will consider how practices of museum \
             curation throughout the twentieth century shaped and defined fundamental categories \
             including the notion of “African art” itself. Students will discuss pressing \
             questions of agency, justice, and power. We will consider early calls from African \
             countries for repatriation of their objects and the ongoing state of these debates \
             today, including the current call for the return of the famed and controversial \
             Benin Bronzes and the efforts of museums like the De Young, the New York \
             Metropolitan Museum of Art, Harvard’s Peabody Museum, and the UCLA Fowler Museum to \
             ethically engage with their African holdings. Throughout the class, our guiding \
             questions will be: Who owns the past? Are these cultural treasures the property of \
             all humanity, as many museums would argue, or of the specific countries and \
             communities who lay claim to them?",
            "The Greeks and Beyond",
            "In this course, we’ll read some foundational works of ancient Greek philosophy, \
             including all or part of Plato’s Symposium, Aristotle’s On the Soul, Sextus \
             Empiricus’ Outlines of Pyrrhonism (the most important extant ancient Skeptical \
             text), and a central Epicurean work, Lucretius’ On the Nature of Things. We’ll focus \
             on four topics: 1. Can I know anything? If so, what can I know and how can I know \
             it? We’ll consider the radical Skeptical view that living without any beliefs is the \
             only route to tranquility. 2. Is my loving someone good or bad for me? What is it to \
             love a person and should we have reasons for loving? What would be a good reason for \
             loving someone? 3. I’m now the same person as one particular baby born in the past. \
             However, there’s little if any overlap between the baby’s matter and my current \
             matter or the baby’s thoughts and my current ones. What makes me the same person \
             over time? Could I, say, by being run through a teletransportation machine twice, be \
             duplicated? Once I understand what makes me the same person over time (if anything \
             does), how should this affect how I treat others? 4. Is my death bad for me? If so, \
             why is it bad for me? It seems entirely obvious that, even if it’s sometimes better \
             for me to die, my dying would, in many circumstances, be bad for me. We’ll consider \
             Lucretius’ famous argument for the claim that no matter when I die–even if I die \
             “tragically early”—my death is in no way bad for me. We’ll find that Lucretius’ \
             argument is surprisingly powerful and that it helps us understand what is good for \
             us. We’ll devote one course meeting to reading some modern philosophy related to \
             each of these topics.",
            "Racial Identity in the American Imagination",
            "From Sally Hemings to Barack Obama, this course explores how racial identity has \
             been experienced, represented, and contested throughout American history. Engaging \
             historical, legal, and literary texts, as well as film, we will examine the major \
             historical transformations that have shaped our understandings of racial identity. \
             We will also explore autobiography, memoir, photography, and music to consider the \
             ways that racial identity has been represented in American society. Our discussions \
             will be guided by questions such as: What is the interplay between racial and \
             American identity? What role do mixed-race identities play in American society? Is \
             race merely a performance? What does it mean for race to be a “social construct”? \
             What roles do class, gender, and sexuality play in the construction of racial \
             identity? Students are expected to approach topics with respect for different \
             backgrounds, experiences and viewpoints, fostering an inclusive learning \
             environment. This course invites students to engage with the complexities of race in \
             America, fostering a deeper understanding of its cultural, historical, and personal \
             significance.",
            "Revolutions",
            "“Revolutions are the locomotives of history,” wrote Karl Marx. As the ongoing \
             turmoil of the Middle East reminds us, revolutions have the power to reshape the \
             political order of the world more than any other social, economic, or cultural \
             forces. Most states today were born out of a revolution. What exactly is a \
             revolution? Is it, like Marx believed, the inevitable result of a social conflict? \
             Or does it take determined revolutionaries to make a successful revolution? To have \
             a revolution, do you have to call it “a revolution”? To answer these and other \
             questions, this course will take students back to the early revolutions of \
             seventeenth-century England, and the revolutions of America and France. We will then \
             make our way through the revolutions of the nineteenth century to the great \
             revolutions of the twentieth century in Russia, China, Cuba, Cambodia, and Iran. We \
             will conclude by considering the recent revolutions in the Middle East.",
            "Spiritual Ecologies: Religion and the Climate Crisis",
            "This course explores how certain religions—Judaism, Christianity, Islam, Buddhism \
             and Hinduism—have addressed the ecological crisis for the past 50 years. The world \
             today is in the midst of a major ecological crisis that is manifested in extreme \
             weather events; loss of biodiversity; depletion of fisheries; pollution of air, \
             water, and soil; prolonged droughts; and mass extinction of species. Since the \
             1970s, world religions have begun to grapple with the religious significance of the \
             environmental crisis, examining their own scriptures, rituals and ethics in order to \
             articulate responses to the ecological crisis.  While respecting the distinctiveness \
             of each religious tradition, this course compares their teachings and examines the \
             following issues: Religion as the cause of the environmental crisis, The resources \
             for ecological responses within each tradition, The emergence of new religious \
             ecologies and ecological theologies, The contribution of world religions to \
             environmental ethics, The degree to which the environmental crisis has \
             functioned—and could function better—as the basis of inter-faith collaboration. \
             Religious traditions can, at times, provide wholesale transferrable solutions to \
             environmental problems, but, as thought-partners and as ethical prompts, they do \
             something deeper: they offer disruptive, radical new ways of thinking, acting, and \
             imagining our place within the nonhuman world.",
        ])),
        notes: vec![],
        ..Default::default()
    }
}

pub fn get_sumac() -> SummerCamp {
    SummerCamp {
        application_fee: Some(65.0),
        deadline: Some("2/3/25"),
        description: vec![
            "SUMaC leads participants on a journey in advanced mathematics through lectures, \
             guided research, and group problem solving. In an environment centered on \
             mathematics, participants explore current lines of mathematical research, the \
             historical development of important areas of mathematics, and applications across \
             scientific disciplines.",
            "Advanced Mathematics: Explore mathematics beyond the scope of what is typically \
             taught in the classroom. SUMaC is for students who have an exceptional interest in \
             mathematics, and who are prepared for study of abstract algebra and number theory, \
             or algebraic topology.",
            "Academic Enrichment: The SUMaC courses are not for credit or grade—they are designed \
             for pure mathematical enrichment.",
            "Intensive Exploration: SUMaC focuses on mathematics that is motivated independently \
             of ties to other sciences; nonetheless, important applications are introduced and \
             pursued along the way.",
            "College-Level Workload: Similar to what they would experience in a college course, \
             participants can expect a heavy and engaging workload of assignments to work on \
             outside of the live class meeting times.",
        ],
        identifier: "Stanford University Mathematics Camp (SUMaC)",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(4.0),
        link: Some("https://sumac.spcs.stanford.edu/"),
        location: Some("Palo Alto, California"),
        organization: Some("Stanford University"),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::UnofficialTranscript,
            Requirement::Custom("Short math questions on the application portal"),
            Requirement::Custom("Online math admissions exam"),
            Requirement::RequiredEssay(
                "Which of the problems on the Admissions Exam did you find the most enjoyable? \
                 Explain.",
                Limit::Words(20..=100),
            ),
            Requirement::RequiredEssay(
                "Which of the problems on the Admissions Exam did you find the most challenging? \
                 Explain.",
                Limit::Words(20..=100),
            ),
            Requirement::ActivityList(
                "List any extracurricular mathematics activities you have participated in (e.g., \
                 math clubs and camps). If none, enter N/A.",
            ),
            Requirement::Custom(
                "List all mathematics courses taken in the last two academic years, including the \
                 current year/term, and the grades you received.",
            ),
            Requirement::Custom(
                "Participation in math competitions is not necessary to be eligible for SUMaC. If \
                 you have participated in any of the following math competitions in the past two \
                 academic years, provide the date, score, and/or ranking: AMC12, AIME, USAJMO, \
                 USAMO, or international equivalents. Do not enter scores from other or older \
                 math competitions. If you have not participated in any of these competitions in \
                 the last two academic years, enter N/A.",
            ),
            Requirement::RequiredEssay(
                "Describe the kind of student you are. What strengths are you proud of? For \
                 example: curiosity, classroom engagement, or dedication, etc. How have you \
                 pushed yourself to grow in your study of mathematics? Include specific examples \
                 from academic experiences.",
                Limit::Words(75..=300),
            ),
            Requirement::RequiredEssay(
                "What are your personal goals for the next five to ten years? What impact would \
                 you like to have on your community, or on society in general?",
                Limit::Words(50..=200),
            ),
            Requirement::RequiredEssay(
                "Describe a memorable experience in which you engaged meaningfully with \
                 mathematics outside of class.",
                Limit::Words(50..=200),
            ),
            Requirement::RequiredEssay(
                "Have you participated in a Stanford Pre-Collegiate Studies program before, such \
                 as: Stanford Math Circle, Stanford Pre-Collegiate Summer Institutes (SI), \
                 Stanford University Mathematics Camp (SUMaC), Stanford Middle School Scholars \
                 Program (SMSSP), Stanford Summer Humanities Institute (SHI), or Stanford \
                 University-Level Online Math & Physics (ULO)? Have you enrolled in a course at \
                 Stanford Online High School (OHS)? If yes, name the program(s) and your dates of \
                 attendance below. If no, enter N/A in the box below.",
                Limit::Words(0..=100),
            ),
            Requirement::OptionalEssay(
                "List all other summer programs you are applying to for this upcoming summer.",
                Limit::Words(0..=100),
            ),
            Requirement::RecommendationForm("math teacher"),
            Requirement::ActivityList(
                "Up to 10 activities. Write activity name, role, frequency of participation, and \
                 200-character description",
            ),
            Requirement::Custom(
                "This video essay is an opportunity to share more about your interest in our \
                 program. You can consider it a conversation between you and our Admissions \
                 Committee. You must record the two minute video essay in the web page in any \
                 number of takes. Answer the question: \"Tell us two things that compelled you to \
                 apply to a Stanford Pre-Collegiate Studies summer program.\"",
            ),
            Requirement::TestScores("Optional, ACT, PSAT, SAT, AP Exams"),
        ],
        tuition: Some(8575.0),
        specialization: Some(make_specializations(&[
            "Program I – Abstract Algebra & Number Theory",
            "Program I topics are introduced through five motivating problems such as limitations \
             of straight-edge and compass constructions, classification of patterns in two \
             dimensions, error-correcting codes, cryptography, and the analysis of symmetry in \
             structures. The mathematics central to solving these problems comes from the areas \
             of abstract algebra and number theory. Abstract algebra originated in the early part \
             of the 19th century through the study of polynomial equations. This branch of \
             mathematics lies at the core of many areas of modern mathematical research. Number \
             theory concerns the properties of integers and has its origins in ancient \
             mathematics. Number theory remains a very active field of study with interesting \
             open problems and important applications in computer science. Recommended \
             Prerequisite Mathematics Experience for Program I: Students applying for Program I \
             should have experience writing and reading mathematical proofs and strong high \
             school geometry and algebra mastery. SUMaC Program I applicants should be \
             comfortable with: proofs by induction, contradiction, contrapositive, and more, \
             logic used in mathematics such as basic logical symbols and their meanings like if, \
             then, or, and, etc, notation for subsets, supersets, and intersections. Students \
             accepted to Program I have typically studied number theory and are comfortable with \
             modular arithmetic and some basic theoretical results involving modular arithmetic. \
             Prior participation in mathematics competitions or contests is not required.",
            "Program II – Algebraic Topology",
            "Program II centers on algebraic topology, a major area of current mathematics \
             research. Topology is the study of the properties of shapes that remain unaffected \
             by deformations. For example, a sphere made out of rubber can be deformed into the \
             shape of a cube. While it may appear that a sphere and a cube don't have that much \
             in common, it turns out that they are topologically equivalent in a way that can be \
             made precise mathematically. This course will explore different ways of analyzing \
             topological properties of shapes using algebraic concepts such as the notion of \
             group. Recommended Prerequisite Mathematics Experience for Program II: Students \
             applying for Program II should have enough mathematics experience to learn new \
             mathematics quickly. SUMaC Program II applicants should have: more proof experience \
             than a Program I student, deep, thoughtful interest in higher mathematics (whereas a \
             Program I student might only have experience from mathematics contests or clubs), \
             experience with group theory, but it is not required. Program II students are \
             typically previous participants of Program I or have experience with content from \
             Program I.",
        ])),
        notes: vec![],
        ..Default::default()
    }
}

pub fn get_summer_institutes() -> SummerCamp {
    SummerCamp {
        application_fee: Some(65.0),
        deadline: Some("3/20/25"),
        description: vec![
            "Not for credit or grade, our challenging courses provide in-depth and interactive \
             exploration of advanced topics. The program emphasizes building skills along with \
             acquiring knowledge and is designed to create environments that foster creativity \
             and collaboration where participants learn from each other as well as from their \
             instructors. Upon completion of the course, participants receive written feedback \
             from their instructor and a Certificate of Completion.",
            "Live Online Instruction: Courses meet for two hours each day (Monday–Friday) for \
             live online classroom discussion between the instructor and students. In addition to \
             the live meeting times, participants complete out of class learning assignments \
             tailored to each course. Visit our course catalog for details about each course’s \
             unique meeting time.",
            "Active Student Engagement: Active participation in class discussions and completion \
             of asynchronous assignments is key to creating an online community of engaged \
             learners. Outside of the academic components of the program, students will also find \
             opportunities to engage and build community with their peers through a variety of \
             student life seminars, workshops, and activities.",
        ],
        identifier: "Stanford Pre-Collegiate Summer Institutes (online)",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        length_wk: Some(2.0),
        link: Some("https://summerinstitutes.spcs.stanford.edu/"),
        location: Some("Palo Alto, California"),
        organization: Some("Stanford University"),
        requirements: vec![
            Requirement::GradeRange(9..=12),
            Requirement::UnofficialTranscript,
            Requirement::RequiredEssay(
                "What interests you most about each of the course(s) you are applying to?",
                Limit::Words(100..=400),
            ),
            Requirement::RequiredEssay(
                "Describe the kind of student you are. What strengths are you proud of? For \
                 example: curiosity, classroom engagement, or dedication. Include specific \
                 examples from academic experiences.",
                Limit::Words(75..=300),
            ),
            Requirement::RequiredEssay(
                "Have you participated in a Stanford Pre-Collegiate Studies program before, such \
                 as: Stanford Math Circle, Stanford Pre-Collegiate Summer Institutes (SI), \
                 Stanford University Mathematics Camp (SUMaC), Stanford Middle School Scholars \
                 Program (SMSSP), Stanford Summer Humanities Institute (SHI), or Stanford \
                 University-Level Online Math & Physics (ULO)? Have you enrolled in a course at \
                 Stanford Online High School (OHS)? If yes, name the program(s) and your dates of \
                 attendance below. If no, enter N/A in the box below.",
                Limit::Words(0..=100),
            ),
            Requirement::ActivityList(
                "Up to 10 activities. Write activity name, role, frequency of participation, and \
                 200-character description",
            ),
            Requirement::Custom("Work sample based on the class enrolled"),
            Requirement::Custom(
                "This video essay is an opportunity to share more about your interest in our \
                 program. You can consider it a conversation between you and our Admissions \
                 Committee. You must record the two minute video essay in the web page in any \
                 number of takes. Answer the question: \"Tell us two things that compelled you to \
                 apply to a Stanford Pre-Collegiate Studies summer program.\"",
            ),
            Requirement::TestScores("Optional, ACT, PSAT, SAT, AP Exams"),
        ],
        tuition: Some(3080.0),
        specialization: Some(vec![(
            "Click to view courses",
            SummerCamp {
                description: vec![
                    "View prerequisites and descriptions on the website",
                    "Art Practice and Career Pathways",
                    "Art, Design, and Technology",
                    "Artificial Intelligence",
                    "Biomedical Engineering",
                    "Business Strategy",
                    "Capital Markets and Investments",
                    "Climate Change: Projections and Uncertainty",
                    "Computer Aided Drafting and Design",
                    "Creative Writing: Creative Nonfiction, Fiction and Poetry",
                    "Design Your Life with Philosophy",
                    "Discrete Mathematics",
                    "Equity in Medicine",
                    "Film Studies: WWII, the Cold War, and the Civil Rights Movement",
                    "Frontiers in Physics",
                    "Game Design",
                    "Innovation and Entrepreneurship",
                    "Introduction to C++",
                    "Introduction to Data Science",
                    "Introduction to Engineering",
                    "Introduction to Ethnic Studies",
                    "Introduction to Finance and Banking",
                    "Introduction to Human-Computer Interaction (HCI)",
                    "Introduction to International Relations",
                    "Introduction to Java Programming",
                    "Introduction to Machine Learning",
                    "Introduction to Structural Engineering",
                    "Investigations in Genetics",
                    "Legal Studies: Critical Thinking Skills",
                    "Logic and Problem Solving",
                    "Media and Politics",
                    "Number Theory",
                    "Philosophy of Artificial Intelligence",
                    "Principles of Macroeconomics",
                    "Product Design",
                    "Strategic Marketing",
                    "The History of Activism",
                    "The Nature of Evil",
                    "Topics in Biochemistry",
                    "Topics in Bioscience",
                    "Topics in Neuroengineering",
                    "Topics in Neuroscience",
                    "Topics in Psychology",
                ],
                ..Default::default()
            },
        )]),
        notes: vec![],
        ..Default::default()
    }
}
