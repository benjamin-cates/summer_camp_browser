use crate::structs::{Limit, Requirement, SummerCamp};

use super::generators::make_specializations;

pub fn get_rmp() -> SummerCamp {
    SummerCamp::default()
}

pub fn get_sra() -> SummerCamp {
    SummerCamp {
        acceptance_rate: Some(10.0),
        link: Some("https://www.summer.ucsb.edu/programs/summer-research-academies/overview"),
        description: vec![
            "UC Santa Barbara's Summer Research Academies offer a dynamic summer experience that \
             engages qualified high school students in project-based, directed research in STEM, \
             Humanities, and Social Sciences fields. Students will take a 4-unit university \
             course in which they choose and develop a research topic specific to the track they \
             select, under the direction of an instructor who is conducting active research in \
             that field.",
        ],
        deadline: Some("3/31/25"),
        application_opens: Some("12/15/24"),
        identifier: "UCSB Summer Research Academies (SRA)",
        length_wk: Some(4.0),
        requirements: vec![
            Requirement::GradeRange(10..=12),
            Requirement::MinWeightedGPA(3.6),
            Requirement::UnofficialTranscript,
            Requirement::RequiredEssay(
                "Please tell us why you want to participate in SRA, the reason you selected a \
                 particular research track(s), and something unique about you for the admissions \
                 committee to know.",
                Limit::Words(0..=500),
            ),
            Requirement::RequiredEssay(
                "Briefly share what you specifically hope to learn from the track(s) you have \
                 chosen.",
                Limit::Words(0..=150),
            ),
            Requirement::RequiredEssay(
                "What skills, interests, or talents do you possess that will enable you to \
                 succeed in the chosen track(s)?",
                Limit::Words(0..=150),
            ),
            Requirement::RequiredEssay(
                "Describe how you will develop your academic and/or personal goals at SRA.",
                Limit::Words(0..=150),
            ),
            Requirement::RequiredEssay(
                "Create a hashtag describing you and/or your life and elaborate on why you chose \
                 it.",
                Limit::Words(0..=150),
            ),
        ],
        application_fee: Some(75.0),
        tuition: Some(2700.0 + 5999.0),
        apply_link: Some("https://ucsbsummer.force.com/apply/s/"),
        last_updated: Some("1/8/25"),
        location: Some("Santa Barbara, CA"),
        organization: Some("University of California, Santa Barbara"),
        specialization: Some(make_specializations(&[
            "Track 1: Complex Systems – Interactions, Inputs, and Networks for Natural and \
             Engineered Systems",
            "After decades of research, scientists and engineers have developed sophisticated \
             mathematical tools to understand and predict complex behaviors across physical, \
             biological, chemical, social, and engineered systems. From the way birds stabilize \
             in flight to how the human body regulates blood pressure, patterns of stability, \
             feedback, and control observed in natural systems inspire the field and theories of \
             dynamical systems. In this course, students will learn how researchers are creating \
             new ways to manage the behavior of everything from robotic arms to autonomous \
             vehicles by studying how systems evolve and respond to disturbances. Students will \
             delve into principles to examine case study applications of biology, robotics, game \
             theory, and more to understand the nature of these systems and uncover control \
             methods. By the end of the course, they will be equipped with the tools to analyze \
             and design complex systems that respond effectively to change.",
            "Track 2: Climate is Lit – The Craft of Narrating a Global Crisis",
            "Climate change is not only a physical reality but also a cultural narrative. As \
             humanity faces a future marked by heightening hurricanes, heat waves, and ecological \
             extinctions, the stories we tell shape the actions we take and the solutions we \
             build. Climate change literature enables readers to explore potential futures, \
             urging them to engage with the complexities of this crisis and its far-reaching \
             effects on both human and ecological systems. In this course, students will research \
             how various genres of literature—including narrative nonfiction, literary \
             journalism, and science fiction—articulate the intricacies of the climate crisis and \
             expose the power dynamics of representation. Students will debate and discuss how \
             race, class, gender, and history influence climate narratives, and to what effect. \
             Through their research, students will acquire tools to strategically communicate \
             climate change to their desired audience in pursuit of meaningful change.",
            "Track 3: Defying Fluidity – Diving Into the World of Bio-Inspired Fluid Mechanics",
            "Most life on Earth began in the ocean, and over millennia, organisms have evolved to \
             manage and harness the power of moving fluids like water and air. Engineers have \
             learned how nature can support the creative design process to develop new \
             technologies ranging from aquatic locomotion to flapping flight to respiratory \
             treatments. In this course, students will learn the fundamentals of fluid mechanics \
             and discover how biological principles can inspire innovative engineering solutions \
             such as shark scale swimsuits and drone systems. Students will analyze case studies, \
             conduct hands-on experiments, and run multiphysics simulations to investigate topics \
             including superhydrophobicity, surface tension, swimming mechanics, lift and drag, \
             and much more. By performing a quantitative analysis of organisms with unique \
             fluid-related adaptations, students will leverage engineering principles to \
             translate nature’s strategies into real-world applications with positive societal \
             impact.",
            "Track 4: Inside Scoop – Mapping Organizational Structures and Networks Through Data \
             Science",
            "Organizations such as small businesses and global corporations are intricate \
             networks of people, resources, and processes that shape daily life. Understanding \
             these networks and corporations provides vital insight into how companies structure \
             themselves, manage resources, and foster collaborations to remain competitive in an \
             ever-changing landscape. This course provides an in-depth exploration of \
             organizational dynamics to understand principles such as hierarchies and leadership, \
             financial resources and innovation flow within networks, and nuances of conflict and \
             negotiation. Through the lens of theoretical and methodological frameworks used by \
             business analysts and organizational researchers, students will work with curated \
             real-world datasets to gain hands-on experience in data science and Python \
             programming. By applying machine learning and network analysis techniques, students \
             will be equipped with critical skills to identify, analyze, and optimize network \
             patterns and model organizational structure, change, and customer interaction.",
            "Track 5: Molecular Clock – Harnessing the Power of Bioinformatics to Reveal \
             Mechanisms Behind Aging",
            "Recent breakthroughs in genetics and molecular biology have increased our \
             understanding of aging. Researchers have identified the mechanisms behind many \
             age-related pathologies, such as an increase in mitochondrial dysfunction, weakening \
             neurons, accumulation of DNA damage, and decline in vascular function, bringing us \
             closer to a universal solution to control aging. In this course, students will use \
             bioinformatics to propose genetic candidates that regulate the aging clock by \
             studying the well-characterized model organism C. elegans. They will investigate how \
             differing genotypes from varying wild isolate strains affect behaviors and \
             longevity. By querying existing databases, they will access information about the \
             entirety of the C. elegans genome and analyze large datasets to test different \
             combinations of relevant genotypes and their behavioral phenotypes. This effort will \
             present new gene classes and mechanisms to pursue as aging regulators that can serve \
             as targets for treatment.",
            "Track 6: Money Moves – Exploring Cultures of Capitalism, Consumption, and Corporation",
            "Capitalism shapes far more than just markets—it influences how we see ourselves, \
             relate to others, and imagine our futures. It is also a powerful cultural force that \
             impacts values, beliefs, and everyday choices in ways that are both obvious and \
             subtle. This course examines how capitalism extends beyond commerce to shape society \
             by exploring how it influences everything from consumer behavior and social media to \
             career choices and the rise of non-traditional jobs, like influencers. Drawing on \
             case studies from corporations, celebrities, and contemporary trends, students will \
             research the cultural effects of economic systems on our sense of identity, \
             community, and humanity. By analyzing capitalism’s intersections with technology, \
             culture, religion, and nature, students will develop an understanding of the \
             cultural dynamics that shape modern life and apply their insights to real-world \
             examples.",
            "Track 7: Quantum Intelligence – Unlocking the Future of Computing and the Power of AI",
            "Quantum computers have proven powerful in solving complex problems significantly \
             faster than classical or supercomputers. Using the power of quantum mechanics, \
             researchers have developed breakthrough algorithms to advance fields such as \
             cryptography, resulting in the chase for qubit supremacy. In this course, students \
             will build on foundational concepts, from qubits to quantum states, to gain a deep \
             understanding of this interdisciplinary field. Renowned algorithms such as Simon’s, \
             Grover’s, and Prime Factorization will form the basis for exploring topics that \
             include system and architecture, mathematical models and their applications, and \
             error correction codes. Students will learn about quantum information, quantum \
             complexity theory, entanglement and measurement, and much more. By the end of this \
             course, students will engage in research to investigate how quantum computing \
             applications push the capabilities and limitations of various systems.",
            "Track 8: Hate Speech – Examining Rhetoric, Influence, and Social Harm in the Digital \
             Age",
            "The exploitation of divisive and inflammatory language proliferates as online \
             platforms infiltrate our lives. Studying the impact of exposure to such rhetoric \
             will help us understand how perceptions of “the other” are shaped in daily \
             conversations. This course investigates hate speech, focusing on how platforms like \
             social media and news sites contribute to its spread. Students will examine how \
             algorithms amplify hateful language, analyze how rhetorical strategies target \
             marginalized groups, and assess the role of bots and fake accounts in escalating \
             hostility. By applying Python, machine learning, and large language models to detect \
             occurrences, students will dissect the relationship between hateful content, \
             ethno-violence, political extremism, and moderation policies' effects on hate speech \
             prevalence. By the end of this course, students will be equipped with the skills to \
             critically evaluate and define solutions that address the societal impacts of hate \
             speech in digital spaces.",
            "Track 9: Code Breaker – Unraveling Genetics from DNA to Cutting-Edge Engineering",
            "From Mendel’s discovery of heritable traits in pea plants to the transformative \
             gene-editing technology, CRISPR-Cas9, genetic researchers have made major advances \
             in the last 150 years. As the blueprint of human bodies, genes influence function \
             including susceptibility to genetic ailments such as Turner Syndrome and Sickle Cell \
             disease. Gene editing has the potential to revolutionize modern medicine by \
             modifying DNA to improve our ability to treat and cure illnesses like cancer and \
             cystic fibrosis. In this course, students will build on the history of genetics to \
             uncover modern discoveries by analyzing and interpreting data from genetic \
             experiments. By studying the model organism Drosophila melanogaster, they will \
             determine how gene disruption affects phenotype and apply genetic techniques to \
             their projects. By the end, students will develop skills in experimental design and \
             gain a deep understanding of genetic principles, preparing them for future \
             scientific exploration.",
            "Track 10: Predictive Modeling – Leveraging Phenomenological and Mechanistic Models \
             to Solve Problems",
            "Predictive modeling is the foundational theory of and underlying technology for \
             modern science and engineering. It can be used as an effective way to organize and \
             simplify our complicated world by forecasting the dynamics of intricate systems in \
             areas such as physics, biology, and social sciences. By capturing behaviors that \
             change over time, dynamical systems describe real-world situations such as the \
             motion of planets, population growth, animal behavior, weather patterns, and the \
             spread of diseases. In this course, students will analyze such systems by \
             identifying mathematical models based on specific data sets. Using Python, they will \
             visualize their models and methods to formulate an optimization problem that makes \
             meaningful predictions with increased accuracy and efficiency. Students will learn \
             about recent breakthroughs in this interdisciplinary field, gain the technical \
             skills to simulate complex systems, and navigate the pitfalls and blind spots in \
             predictive modeling.",
            "Track 11: Policy Puzzle – Piecing Together the Underlying Principles Behind \
             Policymaking",
            "Public policies shape our lives multifacetedly, influencing economic, political, \
             societal, and individual outcomes. Analyzing these impacts is critical in \
             identifying solutions, assessing their effectiveness, and understanding wider \
             effects. This course dives into the foundational principles of interdisciplinary \
             social science research through a focus on empirical analysis, including \
             qualitative, quantitative, and mixed methods, by exploring applications across \
             diverse substantive areas, such as economic growth and structural change, human \
             welfare, population growth and health, labor markets and migration, education and \
             social policy, and environmental policy. Students will critically assess the \
             limitations of current methods and theories used to evaluate policy impact on key \
             issues such as international trade, industrialization, and globalization by \
             connecting research findings to real-world challenges. By the end of the course, \
             students will develop the skills to conduct original, theoretically informed \
             research, equipping them to effectively analyze, evaluate, and communicate findings \
             to inform public policy and law.",
            "Track 12: Digital Brain – The Science Behind How New Media Is Shaping Our Brain and \
             Behaviors",
            "In today’s digital world, our brains are constantly engaging with media—from social \
             scrolling to immersive gaming. Scientists have long sought to understand how this \
             consumption alters our brains, thoughts, and behaviors. As digital media becomes \
             increasingly integral to daily life, concerns grow about its effects on brain \
             development in children and teens. This course addresses the effect of this medium \
             on mental health and disorders such as addiction through three core areas: \
             neuroscience research on the brain systems of reward and inhibition, the \
             foundational psychological models of learning and development, and communication \
             theories on media impact. Students will conduct MRI scans, analyze imaging data, and \
             collect survey responses to investigate the consequences of digital technology on \
             brain health and behavior. By the end of this course, students will be equipped to \
             think critically and contribute to scholarship on a critical topic currently \
             impacting policymakers, educators, and families.",
        ])),
        ..Default::default()
    }
}
