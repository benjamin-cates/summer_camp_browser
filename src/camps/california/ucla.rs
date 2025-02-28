use crate::structs::{Limit, Requirement, SummerCamp};

fn make_specializations(strs: &[&'static str]) -> Vec<(&'static str, SummerCamp)> {
    strs.chunks(4)
        .map(|chunk| {
            (
                chunk[0],
                SummerCamp {
                    description: vec![chunk[1]],
                    length_wk: Some(chunk[3].parse().unwrap()),
                    link: Some(chunk[2]),
                    ..Default::default()
                },
            )
        })
        .collect()
}

pub fn get_institutes() -> SummerCamp {
    SummerCamp {
        deadline: Some("6/13/25"),
        description: vec![
            "As the only official academic precollege programs offered by UCLA, Precollege Summer Institutes provide motivated and driven high school students the opportunity to earn college credit in one to three weeks while advancing their academic career and abilities in an area of study of their interest. Through co-curricular components including hands-on projects and performances, field visits, and guest lectures, students receive a comprehensive and immersive study of their chosen subject that goes beyond classroom instruction. Our Precollege Summer Institutes are open to high school students from around the world. We invite you to join our diverse and inclusive UCLA community full of respect, ideas, and optimism.",
        ],
        identifier: "UCLA Summer Institutes",
        keywords: vec![],
        last_updated: Some("1/11/25"),
        link: Some("https://summer.ucla.edu/summer-programs/precollege-summer-institutes/"),
        location: Some("Los Angeles, California"),
        organization: Some("University of California, Los Angeles"),
        requirements: vec![
            Requirement::RequiredEssay("At the time of registration, ALL applicants will be prompted to submit a few short sentences reflecting on their pursuit of participation in a UCLA Precollege Summer Institute.", Limit::Unspecified),
            Requirement::UnofficialTranscript,

        ],
        specialization: Some(make_specializations(&[
            "Acting and Performance Summer Institute",
            "The UCLA Acting and Performance Summer Institute is a three-week, intensive program for high school students in theater arts. The program encompasses performance training classes, movement-based techniques, and a final showcase where students create their own unique content through the devised theater process. Each morning begins promptly with tai-chi exercises, followed by acting and movement classes. Other classes may include but are not limited to: classical acting, combat, acting for the camera, and playwriting. In the afternoon, the performance workshop provides students with practical experience in the rehearsal and performance process. Students are involved in all aspects of the creative process – conceptualizing, writing, and transforming ideas into dramatic action. There will be a final showcase for invited guests. This program is designed for high school students with a commitment to the theater arts who seek the discipline and training required for participation in a university theater program or a career in the performing arts.",
            "https://summer.ucla.edu/program/acting-and-performance-summer-institute/",
            "3.0",
            "Applications of Nanoscience Summer Institute",
            "The Applications of Nanoscience Summer Institute is designed for high school students with a background in chemistry and a desire to learn the basics of pursuing a viable, scientifically-sound technology and bringing it to market. As a model, students will explore a few important applications of nanoscience while also learning the basics of reviewing existing scientific literature, design-thinking, and entrepreneurship. They will then go through the process of proposing a product in pursuit of bringing their nanotechnology-based idea to market, and perform preliminary research to validate the science behind their idea. During this two-week program, students have the unique opportunity to explore questions similar to those currently investigated by the scientific community. The program involves the combination of vigorous scientific methodologies and techniques with business projects that are both fun and exciting. At the end of this course, students will demonstrate their newfound knowledge by working on a team project to experience the process of team research and pitching a business and a product to technology investors.",
            "https://summer.ucla.edu/program/applications-of-nanoscience-summer-institute/",
            "2.0",
            "Art Summer Institute",
            "Study at the world-renowned UCLA Department of Art this summer! The Summer Art Institute is designed for talented and highly motivated high school students. The program offers three sessions, two in-person and one virtual, with various specializations to choose from and is structured around a combination of daily, focused studio work, artist lectures, group critiques, and one-on-one meetings with faculty. Students may enroll in one or more session . Students should expect an intensive two weeks of immersive studio instruction and be prepared to work hard.",
            "https://summer.ucla.edu/program/art-summer-institute/",
            "2.0",
            "Camera Acting Summer Institute",
            "The UCLA Camera Acting Summer Institute is a two-week, intensive program for students interested in the performing arts. This program encompasses voice, scene study, storytelling, and on-camera acting training.  Students will learn about the process of relating to the camera lens and approaches to understanding a script so that specific choices can be made to prepare for and connect in the audition room. Students will learn the technical components of acting for the camera and writing and filming their own content, while also gaining hands-on experience executing a self-taped audition. Beyond the classroom, students will attend guest workshops and learn the art of collaboration. This program is designed for high school students who seek the discipline and training required for participation in a university on-camera program or a career in the entertainment industry.",
            "https://summer.ucla.edu/program/camera-acting-summer-institute/",
            "2.0",
            "Critical Thinking Summer Institute",
            "Join us for the Critical Thinking Summer Institute, a premier program for motivated high school students seeking to enhance their critical thinking skills and explore cutting-edge topics in today’s media landscape. Led by expert instructors, our program offers a comprehensive introduction to the theory and practice of critical thinking, including the application of logic and probability theory to evaluate arguments and the responsible and creative use of AI. Enrolled students will also have the opportunity to earn credit in Phil 9 (Principles of Critical Reasoning) and participate in an exclusive workshop on media literacy and journalism, run by industry-leading experts from the prestigious University of Queensland. In this workshop, you will learn to assess media, with a particular focus on new media, and gain firsthand experience in preparing an article for publication. Don’t miss out on this incredible opportunity to hone your critical thinking skills, gain invaluable experience in media literacy and journalism, and prepare for success in the digital age!",
            "https://summer.ucla.edu/program/critical-thinking-summer-institute/",
            "3.0",
            "Design Innovation Summer Institute in Costume Design",
            "The Design Innovation Summer Institute in Costume Design is a UC credit-bearing intensive for students interested in costume design for theater, film, and television. This program provides opportunities to work with our distinguished faculty in classes such as Costume 101, Introduction to Costume Sketching, Silhouette, Sculpture and Costume Design for Theater, Film and Television. The combination of these courses will allow students to use the training to bring stories to life through their designs. Participants will learn the technical components of breaking down a script and the art of collaboration with directors while gaining hands-on experience in research, sketching costumes, and creating mood boards and color palettes. The three-week version of this institute (Costume Design+, Session A) offers training in both costume design and the world of shoemaking, hair and make-up. The two-week version (Costume Design, Session B) focuses exclusively on costume design. Beyond the classroom, students will attend guest workshops with industry leaders that will teach students how to pursue costume design in film and cover portfolio preparation.",
            "https://summer.ucla.edu/program/costume-design-summer-institute/",
            "3.0",
            "Design Innovation Summer Institute in Lighting and Sound",
            "The Lighting and Sound Summer Institute is a two-week, UC credit-bearing intensive for students interested in lighting and sound for theater. This program provides opportunities to work with our faculty in classes such as Introduction to Lighting Intensity, Form, and Direction, Electrics Programming, and Drafting for Theater. Students will learn the technical components of breaking down a script and the art of collaboration while gaining hands-on experience in curating research, communicating ideas, and expressing creative intent. This program is designed for a student at any level who seeks the additional discipline and training required for participation in an educational theater setting and a career in the entertainment industry.",
            "https://summer.ucla.edu/program/design-innovation-summer-institute-lighting-and-sound/",
            "2.0",
            "Design Innovation Summer Institute – Scenic",
            "The Design Summer Institute in Scenic Design is a two-week, UC credit-bearing intensive for students interested in scenic design for theater, film, and television. This program provides opportunities to work with our distinguished faculty in classes such as Introduction to Scenic Design and Scale Drafting, Production Design for Theater, Film and Television, and Storyboarding. Students will learn the design process for scenic design for theater and be introduced to production design. The combination of these courses will allow students to use the training to discover how we can tell stories through visual research, sketches, and model design  Students will learn the technical components of breaking down a script and the art of collaboration with directors while gaining hands-on experience such as model-building, perspective drawing, and scale measurement as means of realizing a design. Beyond the classroom, students will attend guest workshops that will cover portfolio and career preparation.",
            "https://summer.ucla.edu/program/design-innovation-summer-institute-scenic/",
            "3.0",
            "Design Media Arts Summer Institute",
            "The Department of Design Media Arts (DMA) at UCLA is one of the nation’s top design departments offering a comprehensive, multidisciplinary education in media creation, which fosters individual exploration and innovative thinking. Geared specifically for high school students, the department offers the DMA Summer Institute, a two-week program with morning and afternoon classes introducing design practices in the contexts of 2D Image, Net, immersive 3D Worldbuilding, and Motion Design. Two versions of the institute are offered: an in-person, residential experience and a virtual/online program. These programs are identical in curriculum design, lesson objectives, and hands-on faculty guidance and support. The program is taught by professionally trained and well-experienced instructors using the most current software and technology. It culminates in a final exhibition and the creation of a portfolio-ready body of work that students may include in their college applications in related fields. It also provides students with a rare opportunity to sample college life in a cutting-edge design department, while earning four units of pass/no pass UC credit.",
            "https://summer.ucla.edu/program/design-media-arts-summer-institute/",
            "2.0",
            "Film and Television Summer Institute – Traditional Animation",
            "Make your own animated film and meet Hollywood’s most accomplished professionals at the world-famous UCLA Animation Workshop. The Traditional Animation track teaches students how to write, storyboard, and draw their own animation film. No drawing experience is required, just limitless imagination! The Traditional Animation Summer Institute will be offered as an online program with synchronous course meetings, practical exercises, and guest speakers. Students will learn traditional hand-drawn animation methods using a Wacom tablet/mouse, and utilize Pencil2D and Adobe Premiere to complete their animated projects. Under the supervision of UCLA faculty, students will complete a series of exercises and learn the animation process from storyboard to post-production. Throughout the course, students will learn about the history of animation as well as contemporary industry practices, and prominent guests from the industry will visit the classroom to share their experiences and answer student questions about their career and working in Hollywood today. Each student will create an animated short film with sound and these final projects will be screened at the conclusion of the program – friends and family are welcome!",
            "https://summer.ucla.edu/program/film-and-television-summer-institute-traditional-animation-track/",
            "1.5",
            "Game Lab Summer Institute",
            "The UCLA Game Lab Summer Institute introduces high school students to game-making as a form of artistic practice, teaching them the techniques and tools that will help them develop analog and digital games that reflect their own creative voice and vision. We now offer two versions of our institute–an in-person, residential experience, as well as a virtual/online program. These programs are identical in terms of curriculum design, lesson objectives, and hands-on, faculty guidance and support. No previous game-making skills are required, but students with an interest in games and in the visual arts, in particular, will find our institute especially stimulating and rewarding. Students in the program develop a solid aesthetic and technical foundation in various aspects of game design–but just as importantly, they begin learning how to express their own, personal ideas through game-making and game art.",
            "https://summer.ucla.edu/program/game-lab-summer-institute/",
            "2.0",
            "Hip Hop / Street Dance Summer Institute",
            "The UCLA Department of World Arts and Cultures/Dance’s Hip Hop/Street Dance Summer Institute is a seven-day program for students who love Hip Hop dance and want to take their technical abilities to the next level. This program is in partnership with Versa-Style: Performance • Education • Community (VS:PEC), a non-profit Hip Hop organization of experienced, world-renowned Hip Hop artists dedicated to empowering the next generation with Hip Hop and street dance learning.",
            "https://summer.ucla.edu/program/hip-hop-street-dance-summer-institute/",
            "1.0",
            "International Development Summer Institute",
            "The UCLA International Development Summer Institute (IDSI) is a three-week, intensive program for high school students passionate about developing innovative strategies to address some of the most critical issues the world is facing today. This program engages students with debates around the widening patterns of disparities of wealth, power, privilege, and access to social justice—as well as the policies, interventions, and forms of citizen engagement intended to address them—both between and within the countries of the Global South and North. Utilizing Los Angeles, a microcosm of the developing world, as a living laboratory, this program offers students a unique opportunity to study, analyze, and critically assess the social, political and economic forces that have shaped inequality in the modern world through an interdisciplinary lens.",
            "https://summer.ucla.edu/program/international-development-summer-institute/",
            "3.0",
            "Mock Trial Summer Institute",
            "Are you a high school student interested in learning more about our nation’s legal system and how attorneys prepare a case for trial? Perhaps you are considering a career in law as a trial attorney, prosecutor, or defense attorney. Maybe you want to build your self-confidence and strengthen your ability to communicate your ideas and opinions to others. If any of these are true, you are the ideal candidate for the UCLA Mock Trial Summer Institute. This week-long program is a great way to broaden your knowledge base, explore college and career options, boost your self-confidence and powers of persuasion – all while making new friends and having fun! Join us online to train with the National Champion UCLA Mock Trial Team Coach and Team Members! Learn effective trial advocacy techniques and improve your public speaking skills. UCLA has won the Collegiate National Championship four times in the last 16 years, more than any other university in the country, and you will have the chance to work with both our coaches and team members.",
            "https://summer.ucla.edu/program/mock-trial-summer-institute/",
            "1.0",
            "Musical Theater Summer Institute",
            "The UCLA Musical Theater Summer Institute is a three-week intensive, conservatory-style program for high school musical theater students encompassing acting, singing, dancing, and guest workshops which culminates in a final showcase. Students will also be introduced to the technical aspects of theater through master classes as it is an essential component of the learning process. The program is designed for high school students completing grades 9 through 12 who have a serious interest in musical theater and who seek the discipline and training required for participation in a university theater program and a career in the performing arts.",
            "https://summer.ucla.edu/program/musical-theater-summer-institute/",
            "3.0",
            "Nanoscale Microscopy Lab Summer Institute",
            "Nanoscale Microscopy Lab is a one-week, hands-on, science learning opportunity for high school students on scientific imaging, a topic that is typically only accessible in an advanced college-level course. Specifically, students will be able to explore three essential microscopy techniques for nanoscience research: fluorescence microscopy, scanning probe microscopy, and electron microscopy. This one-week summer course offers an exploratory introduction to this important scientific skill for students as early as 10th grade at the high school level. For participating students who intend to become science majors in college, this course will provide foundational knowledge of a key element in scientific observation methods. For participating students who might not pursue a science career, the course offers a glimpse into how scientists study the invisible world of atoms and molecules, which enables key technical innovations, from novel drug delivery methods to advanced energy and computational hardware systems.",
            "https://summer.ucla.edu/program/nanoscale-microscopy-lab-summer-institute/",
            "1.0",
            "Nanoscience Lab Summer Institute",
            "The Nanoscience Lab Summer Institute is an exclusive summer workshop for high school students with a background in chemistry who are interested in advanced science and technology. During this five-day program, students have the unique opportunity to explore technologies similar to those currently investigated by the scientific community through hands-on experiments that give students a survey of diverse topics, including bio-toxicity, supercapacitors, and photolithography. These experiments, designed by UCLA researchers, teach students the key concepts of nanoscale phenomena that make nanoscience and nanotechnology one of the most exciting fields of research today.",
            "https://summer.ucla.edu/program/nanoscience-lab-summer-institute/",
            "1.0",
            "Pit Orchestra Summer Institute",
            "The UCLA Pit Orchestra Summer Institute is an immersive two-week program designed to equip high school musicians with the skills and knowledge essential for thriving in the dynamic world of playing in a pit orchestra. Participants will embark on a musical journey, learning the intricacies of preparing a musical production, refining sight-reading abilities, exploring the music business and various platforms in technology, and honing a wide array of musical and collaborative skills. From rehearsal techniques and orchestration to networking in the industry, this program offers a comprehensive exploration of the pit orchestra landscape.",
            "https://summer.ucla.edu/program/pit-orchestra-summer-institute/",
            "1.0",
            "Sci | Art Lab + Studio Summer Institute",
            "In the highly competitive Sci|Art Studio + Lab, students are immersed in science and art practices to simultaneously develop and sharpen analytical and creative skills. This program prepares students for interdisciplinary thinking before they begin their undergraduate education. During this intensive two-week program, students make connections between cutting-edge scientific research, popular culture, and contemporary arts. Through historical retrospectives, surveys of current art-science collaborations, and science fiction movie screenings, students are exposed to the interface of science, art, and culture with a focus on multidisciplinary collaborations.",
            "https://summer.ucla.edu/program/sciart-lab-studio-summer-institute/",
            "2.0",
            "Stage Management Summer Institute",
            "The UCLA Stage Management Summer Institute is a three-week intensive conservatory-style program for college and high school theater students encompassing the study and application of stage management, theater history, leadership, teamwork, and problem solving. Students will also be introduced to current industry practices through workshops and hands-on activities as it is an essential component to the learning process. Students will spend the morning sessions breaking down stage management at each step of the theatrical process. We will look at not just how to get something done but why it’s done that way, how it was done in the past, who it effects, and how it makes the process better. Students who participate in Session A will culminate the program by working with an ensemble on a devised theatrical piece and those who enroll in Session B will work on a musical. Both sessions will require student to be an active contributor from preproduction thru performance. The program is designed for high school students who have a serious interest in stage management and who seek the discipline and training required for participation in a university theater program or a career in the performing arts.",
            "https://summer.ucla.edu/program/stage-management-summer-institute/",
            "3.0",
            "Startup UCLA: Social Entrepreneurship Summer Institute",
            "The Startup UCLA: Social Entrepreneurship Summer Institute will introduce students to social entrepreneurship, including starting social enterprises and non-profits. In this hands-on program, students will design their own social good organization. They will learn the basics of starting, building, and running non-profit organizations and social impact businesses. The program will focus on the creation of new organizations designed to contribute to the greater good. Students who are interested in learning how to start their own organization for social good or leading social impact movements are encouraged to apply!",
            "https://summer.ucla.edu/program/startup-ucla-social-entrepreneurship-summer-institute/",
            "2.0",
            "TeenArch Studio Summer Institute",
            "The TeenArch Studio Summer Institute program is offered by the UCLA Department of Architecture and Urban Design. This program is an intensive three week-long, full-time summer architecture experience for high school students interested in exploring architecture and design thinking. Structured around the experience of a design studio typically offered within an undergraduate curriculum, the program exposes high school students to the architecture discipline, giving them a sense of the life of a college student. Students will get the chance to experience the architecture profession through the design of a project plus more and receive mentorship from current UCLA Architecture and Urban Design students, alumni, and faculty.",
            "https://summer.ucla.edu/program/teen-archstudio-summer-institute/",
            "3.0",
        ])),
        notes: vec![
            "Commuter camps omitted"
        ],
        ..Default::default()
    }
}

pub fn get_scip() -> SummerCamp {
    SummerCamp {
        application_fee: Some(0.0),
        application_opens: Some("1/3/25"),
        deadline: Some("Early action: 1/31/25, Official application: 4/1/25"),
        description: vec![
            "The UCLA Summer College Immersion Program (SCIP) offers exceptional high school \
             students the opportunity to experience a comprehensive introduction to college life \
             while studying alongside UCLA undergraduates. Designed to emulate the undergraduate \
             experience, SCIP provides students access to a broad range of UCLA’s academic \
             expertise and resources through lectures, workshops, and seminars, as well as UCLA \
             coursework. SCIP’s co-curricular components are specifically curated to empower \
             students to gain balance and thrive as they embark on their college journey. \
             Successful participants will earn college credit, a certificate of completion, and \
             the skills to successfully manage college applications, academics, and university \
             life.",
        ],
        identifier: "UCLA Summer College Immersion Program (SCIP)",
        last_updated: Some("1/18/25"),
        length_wk: Some(6.0),
        link: Some(
            "https://summer.ucla.edu/summer-programs/summer-college-immersion-program/scip/",
        ),
        location: Some("Los Angeles, California"),
        organization: Some("University of California, Los Angeles"),
        requirements: vec![
            Requirement::GradeRange(11..=12),
            Requirement::AgeRange(17..=19),
            Requirement::MinUnweightedGPA(3.8),
            Requirement::UnofficialTranscript,
            Requirement::Custom("Resume"),
            Requirement::RequiredEssay(
                "Answer either: 1. Provide a brief summary of your personal, academic, and/or \
                 professional goals and how SCIP would help you in your pursuit of said goals; OR \
                 2. Tell us how your community, family, and/or culture has shaped you into the \
                 person you are today.",
                Limit::Words(350..=500),
            ),
        ],
        tuition: Some(10000.0),
        notes: vec!["Chose 1 or 2 classes from this list: https://summer.ucla.edu/courses/approved-scip-courses"],
        ..Default::default()
    }
}
