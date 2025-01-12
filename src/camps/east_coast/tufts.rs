use crate::structs::{Limit, Requirement, SummerCamp};

fn spec(
    name: &'static str,
    desc: &'static str,
    length: f64,
    reqs: Vec<Requirement>,
) -> (&'static str, SummerCamp) {
    (
        name,
        SummerCamp {
            description: vec![desc],
            length_wk: Some(length),
            requirements: reqs,
            ..Default::default()
        },
    )
}

pub fn get() -> SummerCamp {
    SummerCamp {
        application_fee: Some(0.0),
        application_opens: Some("12/2/24"),
        deadline: Some("Rolling 5/1/25"),
        description: vec![
            "Choose from a variety of 1 to 3-week programs designed for high school students to \
             explore relevant topics taught at a college-level by Tufts faculty.",
        ],
        identifier: "Tufts University Precollege Intensive",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        link: Some(
            "https://universitycollege.tufts.edu/pre-college/academics/high-school-summer-programs",
        ),
        location: Some("Medford, Massachusetts"),
        organization: Some("Tufts University College of Lifelong Learning"),
        requirements: vec![
            Requirement::OfficialTranscript,
            Requirement::LetterOfRec("anyone"),
        ],
        specialization: Some(vec![
            spec(
                "International Relations",
                "Welcome to the Pre-College International Relations Program! This is a \
                 particularly fascinating time to study international affairs. With a global \
                 pandemic and current global conflicts creating unprecedented challenges for \
                 international travel, trade, economics, and global public health, we are relying \
                 on experts in international affairs and policy to help solve the crisis. With \
                 our International Relations intensive, you will be immersed in a cohort of peers \
                 and world-class faculty who share your passion for solving problems on a global \
                 scale. During the program, rising high school juniors and seniors will \
                 experience Tufts University’s distinctive approach to the study of International \
                 Relations, earn college credit, and create a path toward future scholastic and \
                 professional goals. Students will analyze current events and theoretical \
                 frameworks for understanding IR through interactive seminars from experts; \
                 interrogate key concepts in small, collaborative discussion sections; begin \
                 developing the language and communication skills that form the foundation of \
                 international affairs; work together in teams to handle simulated crisis \
                 situations; conduct research using academic resources available from Tufts’ \
                 Tisch Library; and compose a college-level research paper with guidance from \
                 advanced classroom assistants and a research librarian. After two weeks of \
                 intensive classes, students will expand their network of peers with a passion \
                 for IR and gain insightful mentors while acquiring a deep familiarity with \
                 collegiate academics and working with Tufts’ highly distinguished faculty and \
                 exceptional resources.",
                2.0,
                vec![
                    Requirement::GradeRange(11..=12),
                    Requirement::RequiredEssay(
                        "Indicate your goal in attending",
                        Limit::Words(0..=250),
                    ),
                ],
            ),
            spec(
                "SMFA Studio Art",
                "Join our community of artists and makers, and spend your summer developing as an \
                 artist. Take art school for a test drive by engaging with SMFA at Tufts’ \
                 interdisciplinary approach to art making and create conceptually sophisticated \
                 artwork. As you learn to work with new materials and deepen your understanding \
                 of familiar mediums, you will make connections to bigger ideas. Over the course \
                 of the program, you will participate in three foundation classes led by a team \
                 of qualified artists and educators: Drawing and Painting, Sculpture/3D, and \
                 Animation. Students will be encouraged to blur the line between disciplines \
                 while advancing technical skills in drawing, painting, sculpture, and animation; \
                 allowing you to create materially complex, intentional works of art. You will \
                 also have the opportunity to connect with the SMFA at Tufts admissions team as \
                 they share insights into portfolio development, college applications, and the \
                 value of attending art school. Group critiques led by instructors, visits to the \
                 SMFA at Tufts art library, and field trips to world-renowned museums, such as \
                 the Museum of Fine Arts Boston and the Isabella Stewart Gardener Museum, will \
                 encourage you to think deeply about what it means to make art!",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Indicate your goal in attending",
                        Limit::Words(0..=200),
                    ),
                ],
            ),
            spec(
                "Leadership for Social Change",
                "Join Tufts University’s Tisch College of Civic Life for an exciting, engaging, \
                 two-week summer program created for those who want to develop their leadership \
                 skills while making a difference in the world and in their communities. The \
                 program will equip students with the civic skills and knowledge to analyze \
                 systemic social issues, work collaboratively, and emerge as change agents who \
                 can inspire others. Just five miles northwest of Boston, Tufts University’s \
                 Medford / Somerville vibrant campus sits on a hill overlooking the city. Tufts’ \
                 location offers the optimal combination of a relaxed and safe campus environment \
                 that is close enough to Boston to take advantage of everything the city has to \
                 offer! Much more than a traditional college readiness program, Leadership for \
                 Social Change will enable participants to work together to develop the skills \
                 and knowledge to enact social change in their own communities — now and in the \
                 future. Join us for your summer studies and share the Tufts experience!",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::AgeRange(15..=18),
                    Requirement::RequiredEssay(
                        "Indicate your goal in attending",
                        Limit::Words(0..=250),
                    ),
                ],
            ),
            spec(
                "Climate Resilience Institute",
                "There has never been a more pressing time to study the intersection of environmental change and social equity to learn how you can make a difference! Students will apply an interdisciplinary case-study approach to the social, ecological, and political facets of contemporary environmental issues; learn the principles and key concepts of environmental change and resilience from experts in many fields; and gain hands-on knowledge of techniques for environmental data collection, analysis and visualization. Topics span from resilient water infrastructure to climate activism and environmental justice. Students will actively participate in community responses to climate change through a series of socio-environmental field trips with Boston-area organizations. Working in small teams, students will identify a socio-environmental research question, gather and analyze data on the subject, and synthesize their findings into a college-level capstone project. The Climate Resilience Institute is hosted by the Tufts Environmental Studies Program.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Indicate your goal in attending",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Business Essentials",
                "Tufts Pre-College Programs Business Essentials is a two-week engaging introduction to business concepts for high school students. Students will explore the role of business in the modern, changing world by learning about different types of business and various roles and functions within business. Students will become confident talking about businesses and business situations while considering how they might engage effectively and successfully in a business in the future—as an employee, manager, entrepreneur, business leader, or change agent. Students will be encouraged to make connections between purpose and profit—successful businesses can be both a force for positive change in the world and financially profitable. This program is run in partnership with Tufts Gordon Institute. For over three decades, Tufts Gordon Institute (TGI) has provided students with the knowledge and skills they need to lead both teams and entire companies. TGI provides students with the practical leadership tools necessary to develop and innovate ideas that will make a difference in the world. Our alumni have become leaders in an array of public, private, and non-profit companies representing all industries from biotech and finance to technology R&D.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Indicate your goal in attending",
                        Limit::Words(0..=250),
                    ),
                    Requirement::RequiredEssay(
                        "What you hope to learn in the program",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Marketing Essentials",
                "Tufts Marketing Essentials is a two-week immersive experience for high school students interested in learning marketing concepts and strategies that they can apply to their own lives and future careers. The skills of marketing touch upon several business and professional domains and can be applied across a range of contexts. From this experience, students will gain an introduction to fundamental marketing concepts and a practical and hands-on approach to the application and use of marketing tools and ideas. They will explore the importance of a marketing orientation and a customer focus and evaluate the range of methods available to the marketer, from traditional marketing to digital marketing. Students will evaluate these various tools and apply them in practice with innovative and exciting practical examples and cases. Our interactive workshops are led by the same professors and thought leaders who teach these concepts to our undergraduate and graduate students. Students will spend time with mentors drawn from Tufts students and alumni who will be their guides to marketing and entrepreneurship learning opportunities and resources at Tufts.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=300),
                    ),
                    Requirement::RequiredEssay(
                        "What you hope to learn in the program",
                        Limit::Words(0..=300),
                    ),
                ]
            ),
            spec(
                "Entrepreneurship & Innovation Bootcamp",
                "The Tufts Entrepreneurship and Innovation Bootcamp is a two-week-long immersive experience for high school students interested in learning how to solve big societal problems through new venture creation. Students will learn how to build a new venture from the ground up, whether it is a startup, small business or non-profit. Students will take part in an authentic Tufts experience focused on Entrepreneurship for youth. They will spend time with mentors drawn from Tufts students and alumni who will be your guides to entrepreneurship learning opportunities and resources at Tufts. They will be working in the very same buildings that some of our most successful alumni entrepreneurs got their start. This boot camp is an in-person, synchronous experiential learning experience open to students all over the world. No prior entrepreneurial experience is necessary – all that’s needed is a drive to make a difference and have impact in the world.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=300),
                    ),
                    Requirement::RequiredEssay(
                        "What you hope to learn in the program",
                        Limit::Words(0..=300),
                    ),
                ]
            ),
            spec(
                "Engineering with Artificial Intelligence",
                "Artificial Intelligence (AI), and the sub-field of Machine Learning (ML), are rapidly becoming an integral aspect of many different technological systems, and revolutionizing both computer science and engineering solutions to many everyday problems. Yet the general understanding of the fields, and the impact of these tools on society, is still not well understood by either the general public nor even fully appreciated by those developing the technology. This program starts the process of being better informed through four main initiatives: (1) background information, historical context, and a mathematical and algorithmic understanding of the technologies, (2) exposure to cutting-edge implementations of AI and ML at both the university and industry levels, (3) debates and discussions about the ethical and societal impact and the roles and responsibilities of the developers in thoughtfully implementing new systems, and (4) hands-on experience of developing new applications and solutions to pressing problems leveraging artificial intelligence and machine learning.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]

            ),
            spec(
                "Engineering Design Lab",
                "The Engineering Design Lab (EDL) program will expose students to engineering, fabrication, robotics, and computation in the context of solving real world problems. Throughout the two weeks, guest speakers from across the university as well as industry representatives will discuss research and development happening within their labs/research groups, as well as issues and problems they face in the real world. These speakers will be a source of inspiration to the students for completing their engineering design projects.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Engineering Investigations",
                "The Engineering Investigations (EI) program is designed to provide an introduction to various different disciplines within the broad field of engineering. Many of the significant challenges facing society involve engineering -- from clean water, to renewable energy, to personalized medicine. As Engineering is a diverse field with multiple sub-fields and many possible career pathways, this program will provide students with an overview of many different disciplines within engineering to broaden their understanding of the field and highlight the multiple ways engineers work and impact the world! The set of Engineering Investigations (EI) activities throughout the two weeks balance presentations, discussions, and hands-on activities to keep the students actively engaged in understanding the fields of engineering while simultaneously getting their own exposure and experiences in doing engineering themselves. In addition to participants hearing from researchers, students, and industry professionals about their work in research, design, and entrepreneurship, students will apply these same concepts and ideas to their own hands-on projects. Each day will include activities designed to introduce and explore fields of engineering such as Civil Engineering, Environmental Engineering, Mechanical Engineering, Biomedical Engineering, Human Factors Engineering, and Electrical Engineering while also considering the concepts of engineering design processes, engineering ethics, client-centered design, engineering project management, and fabrication.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Engineering of Music",
                "Music is a universal language. It helps people to understand and express themselves, communicate and connect with others, and transcend cultural barriers. While many of us recognize these as truisms, the role of engineering in music, and the exciting research and career opportunities at the intersection of these and other fields, are less widely known. Over the course of this two-week program, students will explore the role of engineering in multiple facets of music creation and delivery, and gain a broader perspective on the potential for meaningful career paths that integrate their academic and extracurricular interests – music or otherwise. This program introduces a range of topics and career pathways in music engineering through three primary modalities: (1) group discussion of relevant math and physics concepts, cultural and environmental impacts, and artistic context, (2) hands-on engineering projects and on-campus field trips to put theory into action, and (3) exposure to guest experts from industry and academia with diverse career trajectories. Students will learn fundamentals of acoustics, record and visualize sound, build simple acoustic and electronic instruments, build loudspeakers, and investigate the acoustical response of music rooms around campus. As the culmination of the program, students will select their favorite topic or activity to develop into a final project, which they will present in a public showcase and document in personal digital portfolios. Whether the experience ignites a lifelong passion for acoustics or just gives a new lens through which to understand applications of engineering, students will come away with a unique and high caliber interdisciplinary experience.",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Mini Med School",
                "Think you want to be a doctor? Mini Med School is a summer intensive that provides driven students who are interested in medical school and other health professions the opportunity to study with one of the nation's leading medical schools: Tufts University School of Medicine. The program is run and designed by faculty from the Department of Medical Education and is instructed by Tufts medical students and faculty. Students will experience what it takes to study medical sciences through lectures on cutting-edge topics and medical case studies coupled with hands-on training, which includes time in the Clinical Simulation Center and advanced anatomy lab at the Tufts medical campus. Question and answer sessions with admissions staff, doctors, medical students, and professionals from other medical fields (such as dentists, veterinarians, and physician assistants) will help participants get a feel for medical school and career options they have in medicine and related health sciences.",
                2.0,
                vec![
                    Requirement::GradeRange(11..=12),
                    Requirement::Custom("1 year of high school biology"),
                    Requirement::AgeRange(16..=18),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Adventures in Veterinary Medicine",
                "The Adventures in Veterinary Medicine High School Program is an engaging and fun way for high school students to spend two weeks learning more about the veterinary profession. This is your opportunity to dig into veterinary medicine through an exciting and intensive program where you’ll be surrounded by others who share your passion for animals, health, and science. Explore specialty fields and important topics in veterinary medicine through informative lectures by Tufts faculty, staff, and vet students and get up close and personal with animals on the Cummings School Farm. ",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::AgeRange(15..=18),
                    Requirement::ActivityList("Animal experience"),
                    Requirement::LetterOfRec("optional second recommneder"),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
            spec(
                "Lab Science Investigations",
                "Are you interested in a laboratory- based experience where you contribute to a research project that studies a critical societal challenge? In the Tufts Lab Science Investigations: Antimicrobial Resistance, you will become a researcher working to understand the challenge of Antimicrobial Resistance (AMR) and antibiotic stewardship. Just imagine a world with no antibiotics! AMR claimed 1.27 million human lives in 2019. If trends continue, drug-resistant diseases could cause 10 million deaths annually by 2050, surpassing diabetes, heart disease, and cancer as a leading cause of human death. As a student researcher in Lab Science Investigations: Antimicrobial Resistance, you will have a uniquely immersive experience with the One Health approach that recognizes that the health of people is closely connected to the health of animals and our shared environment. In the case of AMR, excessive use of antibiotics in agriculture, animals, and people, all contribute to this problem.. To address this, as part of the STEM Research Institute, you will hear perspectives from experts in many fields and you will work with laboratory research techniques used by engineers, social scientists, biomedical scientists, veterinarians, physicians, drug developers, epidemiologists, healthcare policy experts, and environmental scientists, who work together at the Tufts Levy Center for Integrated Management of Antimicrobial Resistance (CIMAR).
",
                2.0,
                vec![
                    Requirement::GradeRange(10..=12),
                    Requirement::AgeRange(16..=18),
                    Requirement::Custom("1 year of high school biology"),
                    Requirement::RequiredEssay(
                        "Statement of interest",
                        Limit::Words(0..=250),
                    ),
                ]
            ),
        ]),
        ..Default::default()
    }
}
