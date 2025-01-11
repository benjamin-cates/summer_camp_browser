use crate::structs::{Limit, Requirement, SummerCamp};

use super::generators::make_specializations;

pub fn get_usc_precollege() -> SummerCamp {
    SummerCamp {
        acceptance_rate: None,
        link: Some("https://www.bu.edu/summer/high-school-programs/rise-internship-practicum/"),
        description: vec![
            "USC’s four-week college immersion program provides high school students like you \
             with university experience in a subject area of their choice. Courses are \
             interactive, engaging and challenging, allowing you to explore new areas of study or \
             build on your high school coursework. Depending on the specific course, you may \
             expect group work, hands-on lab experience, field trips, guest speakers, and/or an \
             encompassing final project. You may register for only one course, as each course \
             requires five to six hours of instruction per day; you won’t have time for more than \
             one. Applications are reviewed as they are submitted, and students are accepted on a \
             rolling basis. If your preferred course is full and you are a qualified applicant, \
             you will be placed on the waitlist for your preferred course and admitted into your \
             second or third choice course.",
        ],
        deadline: Some("Rolling 5/2/25"),
        identifier: "USC Precollege Programs",
        length_wk: Some(4.0),
        application_opens: Some("10/28/24"),
        requirements: vec![
            Requirement::GradeRange(10..=12),
            Requirement::RequiredEssay(
                "Answer ONE of the prompts\n 1. Describe a topic, idea, or concept you find so \
                 engaging that it makes you lose all track of time. Why does it captivate you? \
                 What or who do you turn to when you want to learn more?\n 2. Discuss an \
                 accomplishment, event, or realization that sparked a period of personal growth \
                 and a new understanding of yourself or others.\n 3. Reflect on a time when you \
                 questioned or challenged a belief or idea. What prompted your thinking? What was \
                 the outcome?\n 4. Some students have a background, identity, interest, or talent \
                 that is so meaningful they believe their application would be incomplete without \
                 it. If this sounds like you, then please share your story.\n",
                Limit::Words(300..=500),
            ),
            Requirement::RecommendationForm("Teacher or Mentor"),
            Requirement::OfficialTranscript,
        ],
        application_fee: Some(65.0),
        tuition: Some(4000.0),
        apply_link: Some("https://usc.smapply.io/"),
        last_updated: Some("1/9/25"),
        location: Some("Los Angeles, California"),
        organization: Some("University of Southern California"),
        specialization: Some(make_specializations(&[
            "Exploration of Architecture",
            "“Exploration of Architecture” is an immersive program that engages you with \
             architecture thinking and the design process. This program will challenge you to \
             connect more deeply with the built world by providing opportunities to experience \
             and create meaningful places through dynamic learning methods. You will imagine and \
             draw, as well as discover, examine, and analyze architecturally impactful places and \
             buildings. You will gain insights into the profession through talks, conversations, \
             and interviews with leading architecture faculty and practitioners. The goal of the \
             program is to stimulate you to pursue further education as architects, designers, \
             and scholars who are leaders in your community and beyond.",
            "Analytics: The Power of Data for Businesses",
            "How does TikTok know what videos to serve you? How is Tesla getting cars to drive \
             themselves? How does Amazon deliver your order the next day? It all comes down to \
             data analysis. In this course, you will learn the fundamental techniques businesses \
             use to answer these questions and get hands-on experience manipulating data \
             yourself. You’ll learn how to organize and clean data so it is ready for analysis, \
             visualize data and draw insights from those visualizations. You will also learn \
             algorithms that businesses use to classify and cluster data to predict future \
             behavior and identify target markets. Ultimately, you will gain a data analyst’s \
             eye, keen to find opportunities to apply these algorithms to the world around you.",
            "Bitcoin & AI: Unlocking the Future of Finance and Technology",
            "This course is a detailed study of the interaction between Bitcoin, our financial \
             system and AI’s accelerating impact on the digital economy. The course begins with \
             monetary history and an overview of today’s dollar-based financial system, then \
             introduces Bitcoin’s origins and technical components, including public-key \
             cryptography and mining. AI and its rapid introduction into the global economy is \
             discussed, as well as the industries it will disrupt for years to come and its \
             intersection with digital assets, including Bitcoin, stablecoins and central bank \
             digital currencies (CBDCs).",
            "Building Sustainable Businesses",
            "Countering global warming and other environmental damage may very well take major \
             changes to key parts of the global economy. Innovators find opportunity in times of \
             change, and many are stepping up to innovate the ways we generate and distribute \
             energy; grow and process our food; move around in our communities and travel over \
             long distances; build our homes; design and manufacture everyday consumer products; \
             and more… We can’t avoid the headlines about intense storms, rampant floods, failed \
             crops and climate refugees. But in this course, we will look at the opportunities \
             that we have to revolutionize our economy and change the relationship between humans \
             and the Earth.",
            "Exploring Entrepreneurship",
            "Have you ever thought about starting your own business? Do you think you have an \
             entrepreneurial mind? In the “Exploring Entrepreneurship” course, you will be \
             immersed in the real world of establishing and building a company. The course blends \
             business theory and the practice of being an entrepreneur.",
            "Introduction to Business",
            "The “Introduction to Business” course provides you with insights on how businesses \
             are managed. A great variety of factors determine a company’s success, including \
             marketing, operations, finance, and leadership, to name a few. This course will give \
             you the basics for understanding how these different divisions work independently as \
             well as rely on each other for a company to prosper. A closer look at selected \
             industries — such as entertainment, sports, and real estate — will provide a deeper \
             understanding of the interplay and influences of these business areas. Students have \
             an opportunity to learn how consumer and economic trends influence how these \
             industries operate and, on the flip side, how these industries can drive consumer \
             interests and economies. The course also offers insights into entrepreneurship — \
             from sole proprietorship examples to exploring niche markets — and much more. The \
             importance of sustainability, creativity and innovation, ethics and corporate social \
             responsibility, and doing business in a diverse world with a diverse workforce will \
             also be discussed.",
            "Discover Engineering",
            "Engineering is a field that impacts every aspect of life, including society, \
             politics, and technology. If you have considered the study of engineering at the \
             college level, the “Discover Engineering” course is an excellent way to preview \
             what’s ahead. You will explore various disciplines of engineering through academic \
             lectures and classroom discussions. You will gain practical experience through \
             hands-on projects that involve designing, building, and testing. You will learn how \
             to conduct research using design thinking and the engineering design process.",
            "Video Game Development",
            "In Video Game Development, students will learn the fundamentals of creating and \
             implementing content in commercial video game engines. Starting with critical \
             learning about existing games, platforms and genres, students will work their way \
             through core development principles to implement those designs in game engines, \
             ultimately making their own playable games. Video Game Development is an opportunity \
             to learn how to bring exciting game ideas to life!",
            "Global Health: Investigating Outbreaks, Preventing Disease",
            "Interested in health and global issues? Want to make a positive change in the world? \
             Become a disease detective this summer! In this course, you will learn about current \
             approaches to promoting health, preventing disease, and delivering health services \
             to communities in need around the world. The course takes a broad, \
             multi-disciplinary approach and explores how medicine, public health, international \
             relations, economics, and other disciplines must be integrated to save lives \
             worldwide. Recent emerging outbreaks, such as COVID-19, Ebola, and Zika, have \
             captured headlines worldwide and illustrated how the intensification of global trade \
             and travel facilitates the rapid spread of disease. The first half of the course \
             will focus on our shared vulnerability to infectious diseases, and you will learn \
             the fundamental approaches used to detect, control, and prevent infectious disease. \
             We will examine historical and current examples of infectious disease outbreaks and \
             learn the strategies used to predict, detect, control, and prevent infectious \
             disease outbreaks locally and worldwide. The second half of the course will focus on \
             how inequity, poverty, gender, climate change, and corporate activity drive poor \
             physical and mental health conditions. We will evaluate disadvantaged and \
             marginalized countries and communities and explore and discuss how human rights law \
             and principles of solidarity and justice can enhance health worldwide. We will also \
             practice using various research approaches, ranging from traditional surveys to \
             photo installations, to generate the evidence needed to advocate for policy changes. \
             We apply a very hands-on, practical approach through fun, daily individual and group \
             exercises, as well as field trips. The course is appropriate for students interested \
             in medicine, public health, anthropology, international relations, international \
             business, economics, law, etc. — in other words, it’s for everyone interested in \
             helping to protect and promote health around the world.",
            "International Relations",
            "Are you interested in what causes war and how we can make the world more peaceful? \
             Are you considering a career in diplomacy, law, conflict resolution, humanitarian \
             aid, environmental protection, human rights, or politics? In the “International \
             Relations” course, you will examine the causes of war and what constitutes \
             successful conflict resolution. You will examine the different stages of conflict \
             and then apply them to the areas of the globe that have experienced war and \
             violence. The course also allows you to study the economic causes of conflict and \
             explore the role of individuals, popular culture, peace initiatives, governments, \
             and international organizations. Throughout, you will develop a case study of a real \
             conflict in the world, examining its causes and proposing a peace agreement and \
             suggestions to resolve the conflict.",
            "Media Literacy in the Age of Misinformation",
            "Democracies relies on enlightened and engaged citizens who can critically analyze \
             the range of information and content disseminated from an equally wide range of \
             sources and platforms. This hands-on, interdisciplinary course teaches media \
             literacy through thoughtful media consumption and creation, empowering students to \
             navigate and ethically contribute to an ever-changing news and media landscape. \
             “Media Literacy in the Age of Misinformation” trains students to become \
             discriminating news and media consumers and contributors by learning the modes of \
             digital media production and distribution. We will explore the economic, technical, \
             and storytelling considerations that go into a making a range of media content, from \
             hard news to viral TikToks. In doing so, students will demystify creators’ \
             motivations, challenge traditional notions of objectivity in journalism, and not \
             only see a piece of content for what is (or isn’t) but make their own intentional \
             and impactful media. The critical thinking skills developed in this course will be \
             applicable to many fields of study, from science to the arts.",
            "Sports Journalism: Multi-platform Storytelling",
            "Learn multi-platform sports journalism at USC Annenberg School of Communication and \
             Journalism in Los Angeles — the epicenter of sports media and entertainment and home \
             to an unparalleled combination of both professional and big-time college sports, \
             including the Dodgers, Lakers, Clippers, Rams, Chargers, Kings, Sparks, Galaxy, \
             expansion LAFC of MLS, and, yes, the USC Trojans! No city or region in the U.S. \
             offers a stronger mix of sports at all levels, even high school — where the No. \
             1-ranked football team in the country, Mater Dei High from nearby Orange County, \
             recently completed an undefeated championship season. The “Sports Journalism: \
             Multi-Platform Storytelling” course emphasizes experiential learning and will \
             combine hands-on, in-class assignments with field trips and in-studio production at \
             the state-of-the-art Annenberg Media Center.",
            "Storytelling in the Digital Age",
            "Today, American democracy faces multiple challenges felt in our communities and \
             institutions. From the videotaped police murder of George Floyd on Memorial Day 2020 \
             to a devastating pandemic, to many Americans rejecting the outcome of a fair \
             election won by President Joe Biden, the foundation of our country is shaken. Our \
             communities’ responses to these challenges will inspire our journey in journalism \
             this summer. In “Storytelling in the Digital Age,” individual and team-reported \
             projects will highlight what’s working and what’s not. You’ll produce stories on a \
             variety of topics, including politics, schools, race, equity, religion, the arts, \
             and culture. You’ll report on the work of unsung heroes of our democracy. You’ll be \
             free to take on lighter fare, too. You’ll enjoy the freedom to stretch and report on \
             stories of your choice. You’ll write journal entries in the first person. You’ll \
             profile people who live in your community. Along the way, you’ll learn storytelling \
             techniques for text, video, and audio platforms. You will gain an understanding of \
             the challenges and opportunities for journalists in the digital world. Your critical \
             thinking and analytical skills will expand. As you master the basic tools of \
             journalism, you will gain an understanding of our world and embrace ways of thinking \
             that will stay with you a lifetime.",
            "Summer Theatre Conservatory: Acting Intensive",
            "The USC School of Dramatic Arts offers an opportunity for you to work with theatre \
             professionals and explore a creative experience in the dramatic arts. This \
             conservatory-style “Summer Theatre Conservatory: Acting Intensive” training course \
             will challenge you and offer a strong foundation in the craft of acting. The \
             program, taught by the school’s world-class faculty, emphasizes process and includes \
             scene study, movement, voice, as well as on-camera acting workshops and a monologue \
             audition master class. The program culminates in a workshop performance for family \
             and friends. Summer Theatre Conservatory is also offered in Comedy Performance and \
             Musical Theatre.",
            "Summer Theatre Conservatory: Comedy Performance",
            "Dreaming up your own Netflix special? Drawn to joining the cast of Saturday Night \
             Live? Desiring collabs and creativity? Start or continue your comedy journey this \
             summer in USC’s dynamic and unique Comedy Performance conservatory. Working with the \
             School of Dramatic Arts’s world-class comedy faculty of experienced professional \
             comedians and comedy actor-writers, you will learn and practice the foundations of \
             improv, stand-up, sketch, sitcom, and physical comedy. While developing partnerships \
             with your comedy cohort, you will explore your own comedic voice, generate original \
             comedic material, and expand your comedic performance skills, culminating in a \
             workshop show for family and friends. This program provides essential training for \
             actors, writers, and humans alike — whether you know you’re funny or not. Summer \
             Theatre Conservatory is also offered in Acting Intensive and Musical Theatre.",
            "Guitar Seminar: Jazz, Rock and Beyond",
            "The “Guitar Seminar: Jazz, Rock, and Beyond” is an immersive musical journey \
             studying the electric guitar at the renowned USC Thornton School of Music and its \
             Studio Guitar Program. This comprehensive program explores every facet of guitar \
             playing and music for young aspiring guitarists. During the seminar, students delve \
             into theory, improvisation, technique, sight-reading, songwriting, and playing \
             repertoire from jazz, rock, and beyond while focusing on stylistic interpretation. \
             The seminar’s hands-on approach includes extensive lab sessions, where students \
             break into smaller groups to practice and perfect new concepts with the guidance of \
             skilled faculty and through clinics conducted by USC’s Studio Guitar faculty. \
             Students will also embark on exciting field trips to Los Angeles’s iconic music \
             venues. There will be no shortage of performance opportunities, from casual \
             showcases of fresh material in front of your peers to evening concerts. The climax \
             of the workshop will be two formal concerts open to family and friends, showcasing \
             the incredible progress and new skills students have acquired. Repertoire and \
             assignments are selected to challenge even the most advanced students, ensuring a \
             transformative and enriching musical experience.",
            "Summer Theatre Conservatory: Musical Theatre",
            "This summer, immerse yourself in music, movement (or dance), and play with the \
             “Summer Theatre Conservatory: Musical Theatre” course! In movement classes, you will \
             be exposed to different styles of dance that will enhance your ability to tell a \
             story physically. In song coaching classes, you will learn how to act the song and \
             feel the music. Acting classes will be devoted to your personal connection to \
             character and text and the basic principles of acting. The USC School of Dramatic \
             Arts experience offers rigorous play and skills to celebrate your unique self \
             through Musical Theatre. Summer Theatre Conservatory is also offered in Acting \
             Intensive and Comedy Performance.",
            "Health and Healing: Explorations in the History of Medicine",
            "When we think of modern medicine, we often imagine a world driven by cutting-edge \
             science. We picture laboratories and hospitals, places where medical researchers, \
             biotech experts, and doctors collaborate to find the causes of and cure disease. \
             What we think about less often is the human side of the healing arts: the way \
             medical care systems are living, evolving practices grounded in specific cultural, \
             religious, historical, and social contexts. If you are interested in pursuing a \
             career in medicine or biotechnology, or have ever just wondered about the \
             fascinating history behind Western healing practices, this course is for you.",
            "Health Innovation: Moving Minds and Bodies",
            "This health innovation course will explore the intersection of health technology, \
             sports science, and the music and film industries. Through special topics, \
             laboratory experiences, guest speakers, and hands-on activities with cutting-edge \
             advancements, you will discover how these fields converge to enhance health and \
             wellness in both athletes and entertainers. Career paths in this innovative space \
             include health technology developer, sports performance analyst, wellness coach, \
             health informatics specialist, music and film wellness consultant, and public health \
             advocate.",
            "Psychological Science and Society",
            "What drives human behavior, and why do we think and feel the way we do? How does \
             psychology explain our interactions with the world, and what can it teach us about \
             creating meaningful, impactful lives? More importantly, how can this science empower \
             you to improve not only your own life but also the lives of those around you? If \
             these questions intrigue you, you’ve found the perfect course. In the “Psychological \
             Science and Society,” we will explore both classic and contemporary psychological \
             theories, examine real-world applications, and develop your skills as both a \
             critical thinker and a budding scientist. You’ll engage in hands-on activities that \
             challenge you to apply these principles to everyday life, build a deeper \
             understanding of human behavior, and reflect on how psychology intersects with \
             societal issues. This course will also equip you with tools to succeed in your \
             academic and career aspirations while fostering your role in creating positive \
             change within your community and beyond.”",
            "The Brain: Introduction to Neuroscience",
            "Are you considering a career in health care, psychology, or the behavioral sciences? \
             Are you otherwise interested in the scientific study of the human brain? In “The \
             Brain: Introduction to Neuroscience,” you will discover some common and unusual \
             aspects of the brain in everyday life and under abnormal circumstances. You will \
             explore normal brain development as well as abnormal occurrences in the brain and \
             their effects on human function and behavior over the lifespan.",
            "The Science of Food, Nutrition, and the Biological World",
            "Food is something we all have some sort of a connection with. Whether you see it as \
             a tool for artistic expression, or simply as fuel for your body, food is derived \
             from the living world around us. “The Science of Food, Nutrition, and the Biological \
             World” course will relate concepts from the biological sciences in an applicable \
             context by using the food we eat to provide students with an understanding of \
             molecular biology, biochemistry, microbiology, health, and nutrition.",
            "Criminal Justice: Law and Punishment",
            "This course will review the past, present, and potential futures of the US criminal \
             legal system. Students will learn about sociological and criminological perspectives \
             on crime, policing, courts, incarceration, substance use and mental health. We will \
             also investigate historical and contemporary responses to the criminal legal system \
             such as the Attica Prison Rebellion, Black Lives Matter, and other efforts to reform \
             and transform punishment.",
            "Legal Reasoning and Argumentation",
            "Have you ever wondered how lawyers think, and why they think differently from other \
             people? Learn the skills necessary for successful legal reasoning and how to relate \
             this knowledge to everyday problems. Attend federal appellate court proceedings and \
             see these skills applied (and misapplied) in real life. At the end of the course, \
             you will have a portfolio of documents that demonstrates your experience with legal \
             analysis and writing. The daily schedule for the “Legal Reasoning and Argumentation” \
             course varies on the days students visit the court; there will be an earlier \
             departure time from campus, and the lunch schedule may be adjusted.",
            "Creative Writing Workshop",
            "This workshop is designed for those with little or no experience in creative writing \
             but eager to give it a try! During our four weeks together, we’ll fully immerse \
             ourselves in the art and craft of creative writing, working collaboratively and \
             doing activities designed to ignite our imaginations. Each week will offer the \
             opportunity to experiment with four genres: the personal essay (also known as the \
             “college application essay”), creative nonfiction, flash fiction, and the short \
             story, respectively. In the process, we’ll learn about the literary techniques of \
             accomplished writers, learn to “read like a writer” and “write like a reader,” and \
             even take a field trip or two to find our inspiration in the city of Los Angeles. \
             You’ll also receive constructive feedback from peers and your instructors to help \
             you to discover your own writing voice. Let’s write!",
            "Ethics in the 21st Century: Business, Politics, & Technology",
            "The world has undergone a social and technological revolution in the 21st century. \
             How should we live in this new world? We are employees, entrepreneurs, voters, \
             scientists, caregivers, consumers, family members, and so on—and also human beings. \
             It can be far from obvious what these roles morally demand of us amid rapid and \
             unpredictable change. In the “Ethics in the 21st Century: Business, Politics, and \
             Technology” course, we will consider urgent moral questions using traditional \
             methods of philosophical inquiry. The questions we will investigate include: What \
             makes a society (un)just? How should we respond to unjust laws? How, if at all, \
             should we address socioeconomic inequality? What immigration policy should a country \
             have?  What do businesses morally owe their employees, shareholders, customers, \
             competitors, and society at large? What limitations, if any, should there be on \
             which goods and services can be bought and sold on the market? What is meaningful \
             work? Might future advances in automated labor justify a universal basic income? May \
             we alter ourselves, our children, and humanity through genetic engineering? How \
             concerned should we be about protecting our data? How much historically human work \
             should be done by artificial intelligences and other machines? This course will \
             prepare students to be more reflective—and, we hope, more ethical—members of \
             society, in addition to equipping them with skills in critical thinking, \
             argumentation, and writing that are highly sought after in the academic, legal, and \
             business world.",
            "The Philosophy of Economic Markets, Money, and Property",
            "We’ll think a lot about money in this course. For example: What is it, and what is \
             its place in the well-lived life? What does ethical investing look like? Should we \
             gamble with our money? Must we give to charity? What are the moral risks and \
             benefits of cryptocurrencies? What sorts of lending practices are morally \
             acceptable? Money is one piece of a much larger market system, and there are lots of \
             interesting philosophical questions to ask about markets themselves. What sorts of \
             things should, or should not, be for sale? When do market transactions become \
             exploitative? What sorts of markets are best? To what extent should the government \
             control the marketplace? What is the best measure of economic health? Do nations \
             have an obligation to trade with one another? We use our money in the marketplace to \
             obtain resources — to come to own things. But what is ownership? What sorts of \
             ownership regimes are possible? What sorts of ownership regimes are best? Is there a \
             limit to how much anyone should own? What things should be publicly owned, and what \
             things should be privately owned? How should societies rectify theft from the \
             distant past? “The Philosophy of Economic Markets, Money, and Property” course seeks \
             to answer these questions and more.",
        ])),
        ..Default::default()
    }
}
