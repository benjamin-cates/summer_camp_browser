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

pub fn get_wharton_global_youth() -> SummerCamp {
    SummerCamp {
        application_fee: Some(100.0),
        deadline: Some("4/2/25, Priority deadline: 1/29/25"),
        description: vec![
            "The Wharton Global Youth Program mobilizes the extensive opportunities of the \
             Wharton academic community to educate and inspire pre-collegiate students to explore \
             business practices, analyze the world’s complex challenges, and take the first steps \
             in becoming leaders who will transform the global economy.",
            "Attend lectures and presentations from outstanding Wharton faculty and guest \
             speakers. Participate in classroom discussions about core business subjects, \
             including research-based theories of leadership and teamwork. Practically apply \
             real-world business practices through Wharton Interactive’s digital simulations and \
             case competitions. Conduct research on business enterprises and speak with \
             successful leaders across the industries of finance, entrepreneurship, \
             entertainment, real estate, retail, and more.",
        ],
        identifier: "Wharton Global Youth Program",
        last_updated: Some("1/11/25"),
        length_wk: Some(2.0),
        link: Some("https://globalyouth.wharton.upenn.edu/"),
        location: Some("Philadelphia, Pennsylvania"),
        organization: Some("Wharton at University of Pennsylvania"),
        requirements: vec![
            Requirement::MinUnweightedGPA(3.3),
            Requirement::RecommendationForm("counselor, teacher, or advisor"),
            Requirement::LetterOfRec(
                "anyone, email additional letters to whartonyouth@wharton.upenn.edu",
            ),
            Requirement::RequiredEssay(
                "2025 essays here: https://upenn.app.box.com/s/s6zgpsmbxjunrrcgpj30sn6lc6lluq3l",
                Limit::Unspecified,
            ),
            Requirement::UnofficialTranscript,
            Requirement::TestScores("optional"),
        ],
        specialization: Some(vec![
            spec(
                "Essentials of Finance",
                "Essentials of Finance provides an introduction to the theory, the methods and \
                 the concerns of the world of finance. Learn about the fundamentals of both \
                 personal and corporate finance. Delve into the topics such as the time value of \
                 money, the trade-off between risk and return, equities and corporate accounting. \
                 Each summer, Wharton Essentials of Finance students: Engage with an exploratory \
                 curriculum inspired by the undergraduate finance program at the Wharton School, \
                 Learn fundamentals of finance with real-world applications and case studies, \
                 Attend lectures and presentations from notable Wharton faculty, lecturers and \
                 PhD students, Collaborate in small teams to complete an analysis project",
                2.0,
                vec![Requirement::GradeRange(10..=12)],
            ),
            spec(
                "Essentials of Entrepreneurship",
                "Essentials of Entrepreneurship provides an immersive introduction to the journey \
                 of creating a new venture. With a two-week schedule filled with lectures, \
                 recitations, and activities, students will learn how a successful startup is \
                 created, as well as obtain core techniques in the areas of user research, \
                 opportunity testing, MVP developing, marketing, scaling and exiting. Ultimately, \
                 the program will challenge students to collaborate with peers, use innovative \
                 thinking, and develop a pitch for an early-stage startup. Whether students plan \
                 to start their own company or become an innovator within an established \
                 organization, this program will encourage students to adopt an entrepreneurial \
                 mindset that can be applied across numerous fields. Each summer, Wharton \
                 Essentials of Entrepreneurship students: Attend lectures and presentations from \
                 notable Wharton faculty, lecturers and PhD students  Learn core techniques in \
                 the area of user research, opportunity testing, MVP developing, marketing, \
                 scaling and exiting  Collaborate in teams to develop a pitch for an early-stage \
                 startup Explore historic Philadelphia and experience college life on Penn’s Ivy \
                 League campus",
                2.0,
                vec![Requirement::GradeRange(10..=12)],
            ),
            spec(
                "Leadership in the Business World",
                "Designed to provide students with a glimpse of Wharton’s undergraduate \
                 curriculum, LBW offers opportunities to learn about leadership in 21st century \
                 organizations through a dynamic and rigorous mix of classes with Wharton \
                 professors and business leaders, real-time business simulations, and \
                 team-building activities. Each summer, LBW students: Attend lectures and \
                 presentations from outstanding Wharton faculty and guest speakers Participate in \
                 classroom discussions about core business subjects, including research-based \
                 theories of leadership and teamwork Practically apply real-world business \
                 practices through Wharton Interactive’s digital simulations and case \
                 competitions Conduct research on business enterprises and speak with successful \
                 leaders across the industries of finance, entrepreneurship, entertainment, real \
                 estate, retail, and more",
                3.0,
                vec![
                    Requirement::GradeRange(12..=12),
                    Requirement::MinUnweightedGPA(3.5),
                ],
            ),
            spec(
                "Management & Technology Summer Institute",
                "M&TSI is an extremely rigorous, non-stop, fast-paced college course that \
                 introduces students to the fundamental knowledge and skills necessary for \
                 successfully linking technology and management concepts in just three weeks. \
                 Highlights of M&TSI include: Courses taught by full-time Wharton and Penn \
                 Engineering faculty Attendees explore Penn’s business and engineering offerings, \
                 hearing guest lectures from prominent faculty recognized as leaders in their \
                 field Students build and present a prototype and go-to-market plan for their own \
                 high-tech venture Project presentations are evaluated by Penn faculty, receiving \
                 feedback from seasoned entrepreneurs and investors Students participate in \
                 simulations led by industry innovators such as Google. Attendees receive a full \
                 Penn college-course credit for successful completion of program",
                3.0,
                vec![Requirement::GradeRange(11..=12)],
            ),
            spec(
                "Moneyball Academy",
                "Sponsored by the Wharton Sports Analytics and Business Initiative (WSABI), the \
                 Moneyball Academy primes students to become leaders in an increasingly \
                 data-driven economy. Taught by Professor Adi Wyner, the Moneyball core \
                 curriculum goes beyond what students will learn in an AP Statistics class in \
                 order to teach students how to apply advanced statistical concepts to sports \
                 analytics. Moneyball covers much of Wharton’s STAT 101 and STAT 470 courses, as \
                 well as pieces of STAT 102, 430, and 471. In addition, students will learn to \
                 read and write code in R, the advanced statistical programming language used by \
                 professional statisticians. By the end of the program, students will be able to \
                 understand and perform many of the sports analyses typically seen in \
                 FiveThirtyEight, Fangraphs, or Hardball Times articles. Furthermore, Moneyball \
                 offers the opportunity to learn from and network with other analytically minded \
                 students. Throughout the program, students will work collaboratively with peers \
                 to complete a final data analytics project, which may be featured in the Wharton \
                 Sports Analytics Journal. They will also hear from guest speakers with \
                 real-world experience in sports analytics. Past speakers include Samuel \
                 Mondry-Cohen, Assistant General Manager, Washington Nationals; Diana Ma, Data \
                 Scientist, Los Angeles Lakers; and Alec Halaby, Vice President of Football \
                 Operations and Strategy, Philadelphia Eagles; among others.",
                3.0,
                vec![Requirement::GradeRange(11..=12)],
            ),
            spec(
                "Data Science Academy",
                "We believe data science is not just a collection of techniques; it is foremost \
                 motivated by real-world problems. The data scientist of the 21st century must be \
                 able to identify relevant problems, provide sensible analyses, and ultimately \
                 communicate their findings in meaningful ways. By the end of the Wharton Data \
                 Science Academy, students will not only be equipped with essential data science \
                 techniques such as data visualization and data wrangling but will also be \
                 exposed to modern machine learning methodologies, which are all building blocks \
                 for today’s AI field. Along the way, students will develop a working proficiency \
                 with the R language, which is among the most widely used by professional data \
                 scientists in both academia and industry. What students can expect: Wharton \
                 instructors who are data science experts will lead the lectures and will also be \
                 available to students outside of class.  Students will advance their skills with \
                 data from real-world cases and will be challenged to articulate their findings \
                 with a final project.  Wharton undergraduate and graduate TAs will engage with \
                 students and share their experiences in studying data science.  Guest speakers \
                 who will share their wisdom of data science as a career.  Students will work in \
                 teams to complete a final project and present to their peers at the end of the \
                 program.",
                3.0,
                vec![Requirement::GradeRange(11..=12)],
            ),
            spec(
                "Product Design Academy",
                "This three week hands-on program is designed for students to research, \
                 brainstorm, prototype, and design a new to the world product through the lens of \
                 the Design Thinking process. Students will gain technical prototyping skills \
                 utilizing the Studios@Venture Labs, a collection of workspaces and labs that \
                 house digital and analog fabrication technologies that empower students to make \
                 their ideas a reality. Students will also be introduced to entrepreneurial \
                 concepts on how to launch their idea and create a business around their product. \
                 Throughout the program, students will utilize the Design Thinking process to \
                 conduct user research to develop ideas for new to the world products – there is \
                 no requirement to already have an idea for a product in mind. Students will then \
                 work in teams around promising ideas to further develop and prototype the ideas \
                 into products as the keystone project for this course. In parallel, students \
                 will work on a series of individual prototyping skills building exercises and \
                 mini-projects to learn CAD design to utilize laser cutting and 3D printing \
                 services.",
                3.0,
                vec![Requirement::GradeRange(11..=12)],
            ),
        ]),
        notes: vec![],
        ..Default::default()
    }
}
