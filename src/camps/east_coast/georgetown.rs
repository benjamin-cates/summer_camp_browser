use crate::structs::{Limit, Requirement, SummerCamp};

fn spec(name: &'static str, desc: &'static str, length: f64) -> (&'static str, SummerCamp) {
    (
        name,
        SummerCamp {
            description: vec![desc],
            length_wk: Some(length),
            ..Default::default()
        },
    )
}

pub fn georgetown_academies() -> SummerCamp {
    SummerCamp {
        acceptance_rate: None,
        link: Some("https://summer.georgetown.edu/all-programs"),
        description: vec![
            "The UC Davis Young Scholars Program is a summer residential research program \
             designed to expose approximately 40 high-achieving high school students to the world \
             of original research within the fields of the biological, agricultural, \
             environmental and natural sciences.",
            "Participants in the UC Davis Young Scholars Program work one-on-one with research \
             faculty and research groups in state-of-the-art laboratories for six weeks. Each \
             student will work on an individual project and prepare a journal-quality paper and \
             symposium presentation about their work.",
            "In addition to scientific research, the UC Davis Young Scholars Program strives to \
             introduce participants to the climate and culture of living and learning on a \
             university campus. Staff make every effort to model the experiences that \
             participants will have during their first years of college.",
            "All participants will be enrolled in five units of University Group Study Credit. \
             Assignments for the program include research notebooks, a written article of journal \
             quality describing the research project and its conclusions, presentation of the \
             individual project at a research symposium and presentation of research to students \
             at the home high school. All work in the program is graded and credit will be \
             awarded after home presentations are completed.",
        ],
        deadline: Some("1/31/25"),
        identifier: "UC Davis Young Scholars Program (YSP)",
        length_wk: Some(6.0),
        requirements: vec![
            Requirement::RequiredEssay(
                "Why do you want to attend the Summer High School Programs at Georgetown \
                 University?",
                Limit::Words(300..=500),
            ),
            Requirement::RecommendationForm("teacher or counselor"),
            Requirement::GradeRange(9..=13),
            Requirement::Custom("Students in the medical academies must be 15 or older"),
        ],
        specialization: Some(vec![
            spec(
                "American Politics Academy",
                "In the American Politics Academy, you’ll take an in-depth look at the three \
                 branches of government through a blend of lectures and debate exercises. You’ll \
                 learn about the important role that political parties, public opinion, and \
                 special interest groups can have on shaping political behavior and decisions \
                 while exploring how America’s changing culture impacts political movements and, \
                 ultimately, federal, state, and local policies. You’ll have opportunities to \
                 hear from key elected officials, Capitol Hill staff, lobbyists, and policymakers \
                 who will join class discussions on congressional campaigns, legislative \
                 priorities, and emerging national issues.",
                1.0,
            ),
            spec(
                "Artificial Intelligence Academy",
                "Today’s computing power and big data are driving the field of Artificial \
                 Intelligence (AI) at unprecedented speed, igniting a new historical moment \
                 between science, ethics, and politics. Through a combination of lectures, guest \
                 speakers, discussion sections, and project building, you will explore not only \
                 the science of AI from its beginnings to its future, but also take a deep dive \
                 into the ethics of AI-enabled automation and the global governance frameworks \
                 taking shape to regulate its use.",
                1.0,
            ),
            spec(
                "Creative Writing Academy",
                "Transform your dreams, ideas, and stories into organized, compelling, creative \
                 written works with dynamic lectures in craft topics, workshop sessions with \
                 graduate student instructors, and insightful, productive feedback from your \
                 peers. This combination of instructional approaches will help you generate and \
                 polish a wealth of new poems, stories, and essays, and allow you to experiment \
                 with innovative forms in the field of creative writing. The Academy will also \
                 focus on the publishing and professionalization aspects of the industry, \
                 exploring what markets are available for your writing, what jobs are available \
                 to creative writers, funding opportunities for your work, undergraduate and \
                 graduate programs in writing, and how to get published. Topics for discussion \
                 will include literary form and targeted craft points, often in relation to \
                 social, political, and environmental themes. In addition, this week-long program \
                 will feature excursions to sites around Washington, D.C., including an exercise \
                 in ekphrastic writing at the National Gallery of Art and the chance to read your \
                 work aloud at Busboys and Poets, a famous D.C. literary hub.",
                1.0,
            ),
            spec(
                "Democracy, Peace & Conflict Academy",
                "In the Democracy, Peace & Conflict Academy, you’ll look in-depth at how \
                 democratic societies deal with conflict through a blend of lectures and \
                 interactive exercises. You’ll learn about the important role of elections in the \
                 peaceful transfer of power and the causes of electoral violence. You’ll debate \
                 why democracies endure and why they erode as you investigate the role of \
                 leaders, civil society organizations, and political institutions. You’ll \
                 practice tools that citizens can use to counter political polarization, such as \
                 perspective-taking, dialogue, and nonviolent communication.",
                1.0,
            ),
            spec(
                "International Relations Academy",
                "As one of the oldest universities in the world, and long considered a hub for \
                 international dialogue, Georgetown is where international leaders often choose \
                 to state their positions and calls for action. During the one-week, \
                 International Relations Academy, you'll have the opportunity to learn from \
                 world-renowned faculty, gain valuable experience, and explore today's global \
                 issues and challenges.",
                1.0,
            ),
            spec(
                "Law Academy",
                "From nationally recognized attorneys and prosecutors to politicians and judges, \
                 many of the nation's lawmakers and legal experts received their education at \
                 Georgetown. As a student in the Law Academy, you will follow in the footsteps of \
                 these distinguished legal professionals and explore your passion for the law \
                 with expert faculty, enthusiastic mentors, and talented peers who share your \
                 interests. Through lectures, guest speakers, and discussion sections, our \
                 program offers not only an introduction to jurisprudence but also a look at how \
                 laws are adjudicated and administered. You’ll examine the most controversial \
                 issues of our day and explore how America's legal institutions change as popular \
                 values, standards, and perspectives evolve. You will also learn from Georgetown \
                 University faculty and other legal experts, and get a chance to sample the \
                 Georgetown undergraduate experience.",
                1.0,
            ),
            spec(
                "Medical Academy",
                "The 1-Week Medical Academy introduces you to the field of medicine and gives you \
                 a glimpse into the Georgetown University Medical Center, which includes the \
                 nationally ranked School of Medicine and School of Nursing & Health Studies, in \
                 addition to the world-renowned Lombardi Cancer Center. You'll explore topics \
                 ranging from human anatomy, physiology, and radiology to surgery, cancer, \
                 biomedical ethics, and procurement of human tissues. You’ll participate in \
                 hands-on labs, including suturing, orthopedic casting, small mammal dissection, \
                 and use of a patient simulator. You will have the opportunity to interact \
                 directly with current medical students, Georgetown University School of Medicine \
                 faculty, and MedStar Georgetown University Hospital physicians to learn about \
                 the path to becoming a physician, discover ways to succeed in medical school, \
                 and explore a variety of medical specialties.",
                1.0,
            ),
            spec(
                "National Security & Intelligence Academy",
                "The National Security & Intelligence Academy brings together unparalleled \
                 government and international relations resources from the Georgetown network to \
                 explore security and intelligence issues. You'll gain a deeper understanding of \
                 how policies enhance the security of Americans in the post-9/11 world, study how \
                 the United States prevents foreign adversaries from gaining access to national \
                 security information, and examine how intelligence information is collected, \
                 analyzed, and disseminated.",
                1.0,
            ),
            spec(
                "Nursing Academy",
                "This summer, spend one week immersed in the fast-paced, challenging, and rapidly \
                 evolving world of nursing. Our Nursing Academy offers an introduction to the \
                 nursing field, where you’ll have the opportunity to explore the fundamentals of \
                 the profession along with a variety of career paths and specializations, such as \
                 adult-gerontology, acute care, family practice, anesthesia, and women's health. \
                 You’ll also examine topics ranging from ethics and cultural competencies to \
                 biological, physical, and social sciences. Throughout the program, you’ll engage \
                 in faculty-led instruction and hands-on clinical experiences using a lifelike \
                 patient simulator at Georgetown’s O’Neill Family Foundation Clinical Simulation \
                 Center.",
                1.0,
            ),
            spec(
                "Washington & the World Academy",
                "With the creation of a new world order after World War II, the United States \
                 came to be the indisputable leader of the so-called Free World. However, over \
                 the past several years, new forces in global and domestic politics have called \
                 into question the role of America in a changing world. This has especially been \
                 the case since COVID-19 pandemic. What will be the future of American \
                 leadership? What will happen to the Atlantic Alliance? Will other great \
                 powers—like Russia and China—fill the possible void? What will be the future of \
                 American ideals, such as democracy and freedom? The Washington & the World \
                 Academy will take you on an engaging journey where you’ll explore the role of \
                 the U.S. in the world and develop a broader perspective of how the changing \
                 dynamics may affect American leadership on a global level. Under the leadership \
                 of faculty from Georgetown’s renowned Walsh School of Foreign Service, you’ll \
                 gain a deeper understanding of the U.S. foreign policy process, examine the \
                 existing and emerging challenges confronting the U.S., and learn how you can \
                 become involved in the field of global affairs.",
                1.0,
            ),
            spec(
                "Biotechnology for Science & Health Academy",
                "The Biotechnology for Science & Health Academy is designed to provide students \
                 with a scientific background the opportunity to conduct experiments to \
                 illuminate the role that Biotechnology plays in the fields of Health and \
                 Science. Background information and concepts are presented in a lecture format \
                 and students then conduct hands-on laboratory exercises to demonstrate the \
                 concepts. Students will experience cloning a gene and analyzing the resulting \
                 product, examining the role of DNA and protein diagnostics in modern healthcare, \
                 how DNA is used in forensics and several other applications of Biotechnology in \
                 a variety of industries. This course will also include applications of \
                 Biotechnology in current industries including Medicine and Health, Forensics, \
                 Agriculture and Food Science, and Basic Scientific Research.",
                2.0,
            ),
            spec(
                "Bridges to Social Justice Academy",
                "This 2-Week Academy serves as an introduction to the multiple pathways that you \
                 can take toward the creation of a more just and humane world. Grounded in both \
                 the Social Change Model of Leadership (SCML) and the Pathways of Public Service \
                 and Civic Engagement framework, this academy will provide you with content and \
                 context to support your understanding of the injustices and structural \
                 oppression perpetuating harm in our society. We will focus on the HOW: the \
                 intersecting and necessary ways to create social change through \
                 community-engaged research; community organizing and activism; direct service; \
                 decolonial philanthropy; advocacy for public policy; and social entrepreneurship \
                 for the common good. Through this exploration, you will develop a self-awareness \
                 of your own strengths and capacities for participation in work for social \
                 change. These two weeks will also provide you the opportunity to explore how \
                 your future college studies and careers can contribute to combating injustice \
                 and building peace. Taking advantage of our location in Washington, DC, you will \
                 learn directly by engaging with diverse stakeholders from colleagues at policy \
                 think tanks to local, national, and international organizations committed to \
                 civil, economic, and human rights. Through site visits, you will also experience \
                 Washington, DC, beyond the monuments, and learn about its rich social justice \
                 history and the city’s current challenges around racism and inequity. \
                 Experiential hands-on activities will provide opportunities for practice in \
                 public speaking, facilitation, project design, and listening, led by \
                 undergraduate mentors and Georgetown faculty.",
                2.0,
            ),
            spec(
                "Entrepreneurship Academy",
                "Explore the advantages and challenges of managing your own enterprise in the \
                 Entrepreneurship Academy. Our two-week program takes an in-depth look at \
                 choosing a start-up, prototype testing, market research, social innovation, \
                 global business, and the operations of running your own business. You will have \
                 opportunities to hear from and interact with experienced entrepreneurs across \
                 different industries to learn about what it takes to become successful in any \
                 career path. Bridging theory and practice, you will learn the basics of design \
                 thinking and brainstorming; mold your ideas into strong, promising business \
                 plans; and participate in teams in a final pitch competition. With an emphasis \
                 on personal development, our program incorporates practical, hands-on experience \
                 to help you build and improve your public speaking and communications skills, \
                 networking strategies, and team-building techniques all while exploring the \
                 world of entrepreneurship.",
                2.0,
            ),
            spec(
                "Forensic Science Academy",
                "In Georgetown's Forensic Science Academy, you'll gain a firsthand look at the \
                 world of forensic science from professionals in the field—including \
                 investigators, detectives, crime scene technicians, and pathologists. Topics \
                 covered will include crime scene diagramming, evidence collection, criminal \
                 investigative process, and video forensic analysis, to the ethical dimensions of \
                 issues faced by today’s criminal investigative professionals. Various exercises, \
                 class interaction, case studies, lectures, discussions, mock crime scene \
                 creation, and examination have been developed to provide exposure to Criminal \
                 Forensic investigation. Test your critical thinking skills by identifying and \
                 interpreting the evidence as you explore the application of science and medicine \
                 in law enforcement.",
                2.0,
            ),
            spec(
                "Marketing & Personal Branding Academy",
                "In this program, students will learn the fundamentals of the field of marketing \
                 through an intercultural lens, at both the business and personal levels. From \
                 traditional marketing to social media and influencing, students will hear from \
                 industry experts about their pathways and career highlights. They will see what \
                 a real-life marketing firm looks like and practice using techniques and tools \
                 used by industry professionals. By the end of the program, students will create \
                 and pitch their own marketing campaigns and learn to better pitch their unique \
                 skills and aspirations.",
                2.0,
            ),
            spec(
                "Sports Industry Management Academy",
                "Georgetown's Sports Industry Management Academy introduces you to the increasing \
                 career options in the sports industry—a $250 billion-a-year business that \
                 continues to be one of the fastest-growing fields in the world. You'll get an \
                 inside look behind the scenes of the industry to explore how events on the \
                 field, in the front office, and behind the camera are intricately linked to \
                 boardrooms, retail locations, and homes around the country.",
                2.0,
            ),
            spec(
                "Business & Leadership Academy",
                "Georgetown's Business & Leadership Academy introduces you to the ins and outs of \
                 the business world. You'll spend your time engaged in a combination of classes, \
                 workshops, simulations, group discussions, and activities preparing you with \
                 practical skills and critical thinking abilities that will benefit you in your \
                 future academic and professional endeavors. This program provides you with a \
                 framework to help you understand important business principles and explains how \
                 to go about recognizing business opportunities, challenges, problems, and \
                 solutions. Each class day will begin with a business briefing discussing \
                 domestic and global issues. To understand a business education framework, it is \
                 important to introduce you to business decision-making, investing, strategy, \
                 finance, management, marketing, international business, law, ethics, \
                 communications, writing a business plan, and, most importantly, leadership. The \
                 Business & Leadership Academy gives you a broad overview of business \
                 fundamentals. The program should be of interest to high school students who want \
                 to pursue a business education at the undergraduate level.",
                3.0,
            ),
            spec(
                "Economics Policy Academy",
                "Georgetown's Economics Policy Academy provides an interdisciplinary exploration \
                 of the complex role played by states and other governing entities in relation to \
                 markets, through the lens of both economics and political science. You will \
                 study theoretical concepts from both fields and practice applying them to \
                 real-world problems—both in the U.S. and abroad—to assess the situations and \
                 evaluate policy solutions. By the end of the program, you will have a deeper \
                 understanding of not only the key philosophical and theoretical concepts behind \
                 an economic policy which affect all facets of our life, such as price \
                 interventions and regulations, but also the main issues the U.S. economy faces \
                 today.",
                3.0,
            ),
            spec(
                "Foreign Policy Academy",
                "Combining lectures, talks by policymakers, visits to a number of foreign \
                 Embassies, crisis simulations, and group discussions, Georgetown's Foreign \
                 Policy Academy introduces you to the many components that inform and create U.S. \
                 foreign policy. Throughout your time in the program, you'll develop your \
                 critical thinking and interpretive skills while examining key international \
                 crises and world leaders' responses and policy choices in a global arena. You'll \
                 also analyze the factors that affect the decision-making process and theories \
                 and methods used to formulate and implement policies. By the time you complete \
                 the program, you'll have a deeper understanding of American foreign and defense \
                 policy, the complexities of terrorism threats, the influence of non-governmental \
                 organizations (NGOs) and think tanks, strategies for aid and democracy-building, \
                 and the underpinnings of foreign commerce and trade policies.",
                3.0,
            ),
            spec(
                "Global Business Academy",
                "Although there is an underlying universality to the basic principles of \
                 business, the application of these principles presents unique challenges to \
                 those involved in international business. The Global Business Academy is \
                 designed to provide a comprehensive and up-to-date overview of global business. \
                 In the program, you’ll explore current drivers of international business \
                 including the globalization of business linking markets, companies, and \
                 financial markets; the growth and importance of technology; and the emergence of \
                 national economies. You’ll also examine institutions important in international \
                 business including the World Bank, the International Monetary Fund, and the \
                 World Trade Organization. Georgetown's Global Business Academy introduces you to \
                 the ins and outs of the business world. Combining investment challenges and \
                 group presentations with dynamic faculty lectures and case studies, our program \
                 prepares you with the practical skills and critical thinking abilities that will \
                 benefit you in your future academic and professional endeavors",
                3.0,
            ),
            spec(
                "Medical Immersion Academy",
                "In Georgetown's Medical Immersion Academy, you'll get a snapshot of a first-year \
                 medical school curriculum while learning from Georgetown University School of \
                 Medicine faculty and medical students. You’ll study topics ranging from cell \
                 biology and pathology to immunology, cardiology, and endocrinology, and you will \
                 learn the science of how human body systems work. You’ll also explore \
                 fundamentals of emergency medicine such as the steps in physical examinations, \
                 positioning and extracting, wound care, control of bleeding, stitches and \
                 suturing, control of bleeding, airway management, CPR, oxygen administration, \
                 medication administration, usage of stents, and splinting. Whether you plan to \
                 be an emergency physician, pediatrician, oncologist, or nurse anesthesiologist, \
                 you will have the opportunity to explore how clinicians of various specialties \
                 use their knowledge to monitor health and cure disease. You'll leave the program \
                 with a deeper understanding of the world of medicine, as well as the tools that \
                 prepare you to pursue medical school.",
                3.0,
            ),
            spec(
                "Neuroscience Medicine Academy",
                "The Neuroscience Academy exposes students to the fundamentals of neurobiology \
                 incorporating a clinical focus from which to explore how the nervous system \
                 functions. Patient cases will be used to Promote discussion of central nervous \
                 system (CNS) injuries, related disorders, and diseases across the lifespan, \
                 Guide your learning of CNS function by studying cellular (neurons/glia) through \
                 systems (sensory/motor/cognitive) anatomy, Facilitate your skills to search for \
                 information using primary, secondary, and tertiary sources, Introduce you to \
                 medical treatments, medication, brain imaging, and related basic science and \
                 clinical research, Enhance your awareness of careers in the medical profession \
                 and undergraduate majors to get you there, Explore topics of neuroethics, and \
                 Help you to develop, contribute, and compete in small groups as a final project. \
                 You'll leave the program with an understanding and appreciation of some \
                 governing principles of nervous system design, function, effect of injury, and \
                 recovery of function.",
                3.0,
            ),
        ]),
        application_fee: Some(40.0),
        tuition: Some(7500.0),
        apply_link: None,
        last_updated: Some("1/10/25"),
        location: Some("Washington DC"),
        organization: Some("Georgetown University"),
        ..Default::default()
    }
}
