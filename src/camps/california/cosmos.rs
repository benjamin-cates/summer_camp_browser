use crate::structs::{Limit, Requirement, SummerCamp};

use super::super::generators::{make_specializations};

pub(crate) fn get_cosmos() -> impl Iterator<Item = SummerCamp> {
    let cosmos_default = SummerCamp {
        organization: Some("California State Summer School for Mathematics and Science"),
        acceptance_rate: Some(10.0),
        application_opens: Some("1/8/25"),
        deadline: Some("2/7/25"),
        length_wk: Some(3.0),
        application_fee: Some(44.0),
        description: vec![
            "COSMOS (The California State Summer School for Mathematics and Science) is a 4-week \
             program for talented high school students hosted by the University of California, \
             Irvine. COSMOS students apply to a specific “cluster”, with each cluster addressing \
             advanced topics in science, technology, engineering, and math (STEM) fields, \
             including a variety of engineering disciplines, pharmaceutical sciences, and \
             computer science (to name a few).  See our clusters for more information.",
        ],
        #[rustfmt::skip]
        apply_link: Some("https://cosmos-ucop.ucdavis.edu/app/main/page/application-and-payment-basics"),
        link: Some("https://cosmos-ucop.ucdavis.edu/app/main/page/campuses-and-clusters"),
        tuition: Some(5256.0),
        last_updated: Some("1/8/25"),
        requirements: vec![
            Requirement::RequiredEssay("Explain why you chose your first choice", Limit::Words(0..=200)),
            Requirement::RequiredEssay("Explain why you chose your second choice", Limit::Words(0..=200)),
            Requirement::UnofficialTranscript,
            Requirement::MinUnweightedGPA(3.5),
            Requirement::LetterOfRec("Prefer STEM teacher"),
            Requirement::LetterOfRec("Prefer STEM teacher, optional"),
            Requirement::GradeRange(8..=12),
        ],
        ..Default::default()
    };
    let cosmos_uci = SummerCamp {
        location: Some("Irvine, CA"),
        identifier: "COSMOS UCI",
        specialization: Some(make_specializations(&[
            "Cluster 1: Exploring the Application of Data Science in the Biomedical Sciences",
            "This cluster explores how data science is used to better understand biological and \
             medical processes. Think about the data recorded from the cell activities of the \
             brain of a patient diagnosed with Alzheimer’s disease. What are the most striking \
             patterns of this data in contrast with the data obtained from a person with normal \
             brain activity? What regions in the brain demonstrate such abnormalities more \
             vividly and is there a way to use data from potentially a large number of patients \
             to build a tool or mechanism for predicting Alzheimer’s in the early stages of its \
             development? How reliable can such a tool be, and is there a way to improve its \
             reliability as we collect more data from more subjects?",
            "Cluster 2: Decoding the Universe: Physics, Big Data, and Computation",
            "Students will embark on a captivating journey through the cutting-edge realm of \
             modern physics. In recent years, the convergence of computing power, machine \
             learning algorithms, and advanced measurement devices has sparked a revolutionary \
             shift in our understanding of the universe. This course is designed to unravel the \
             mysteries behind these breakthroughs and explore how recent technological \
             developments have reshaped our perception of the cosmos and the fundamental nature \
             of matter.",
            "Cluster 3: Tissue and Tumor Biology and Mathematical/Computational Modeling",
            "Students will explore, through lectures and virtual laboratory practicums, the \
             biological mechanisms regulating growth of living cells and their cooperative \
             interactions in tissues. Letter topics will include descriptions of the cell cycle \
             and its control points, signal transduction pathways in growth and development, \
             genetic mutations involved in carcinogenesis and limiting parameters on tumor \
             growth. Students will also learn about the applications of cutting-edge analytical \
             instruments, e.g., confocal fluorescence microscopes. For laboratory segments, \
             students will learn about how simple model organisms (Xenopus frog eggs, Hydra) and \
             tissue culture experiments are used to study aspects of normal cell behavior in \
             development (rapid cell division, morphogenesis and pattern formulation). The \
             experimental data will be compared to the results from mathematical models for \
             growth control, differentiation and morphogen signaling developed in the \
             mathematical biology course that are relevant to tumor biology.",
            "Cluster 4: Explorations of the Subatomic World and the Expanding Universe",
            "This course will introduce students to the expanding universe and the fundamental \
             particles that create the periodic table of elements. At the end of this course, \
             students will appreciate how fundamental particles interact with each other and how \
             astronomers infer the expansion rate of the universe. Building on this \
             understanding, the course will discuss the big open questions that astronomers and \
             particle physicists are working on.",
            "Cluster 5: Sustainable Aviation Systems",
            "The aviation industry is critical to the global economy and continues to incorporate \
             new technologies and vehicles each year. As a growing sector, aviation faces a \
             modern sustainability challenge, and aerospace engineers who design aircraft are \
             responsible for both improving performance to handle increasing demand and reducing \
             the environmental impacts associated with the industry.",
            "Cluster 6: Genes, Genomes, and Skeletal Muscle Dystrophies",
            "Genes manage all aspects of living organisms. While the genes in most cells in an \
             organism are identical, cells can differ greatly in their morphology and function. \
             This cluster will analyze genomes and focus on how different levels of gene \
             expression help us differentiate between cells and organisms. The development and \
             dystrophies of skeletal muscle will be used as a case study throughout. Through data \
             analysis, labs, and computer modeling, students will have the opportunity to apply \
             new sequencing technologies that are revolutionizing biology and medicine today.",
            "Cluster 7: BioEngineering Your Brain: Controlling the World with Your Brainwaves",
            "Translating thoughts into actions without acting physically has always been an idea \
             of which dreams and science fiction stories are made of. Recent developments in \
             brain-computer interfaces (BCI), however, have opened the door to making these \
             dreams come true by helping patients who cannot move be able to interact with their \
             world using their brainwaves. The goal of this cluster is to introduce BCIs to \
             students and the underlying engineering and neuroscience principles. Through \
             lectures and hands-on activities, students will learn important neuroscience \
             concepts and analyze biophysiological signals to control devices with BCIs.",
            "Cluster 8: BioEngineering and Characterizing Human Skin Organoids",
            "One of the most cutting-edge fields in medicine is focused on growing human tissues \
             or organs to study human biology and aid in drug development. From lowering \
             cholesterol to wiping out cancer, these human “organoids” mimic human tissues and \
             partly drive a multi-billion-dollar pharmaceutical industry. In this \
             interdisciplinary course, we will focus on bioengineering human skin organoids in a \
             lab. You will learn how to bioengineer three-dimensional scaffolds to grow tissues \
             and to formulate hypotheses on how spatial position or growth conditions can \
             influence tissue growth. You will also learn the biology and engineering behind the \
             organoid-making process, design and engineer your own tissue scaffold, and carry out \
             assays to test how well your tissue grew. This hands-on course will explore the \
             basics of tissue design and how simple cells weave complex webs to maintain the \
             engine of the organ.",
            "Cluster 9: Stressed Out bugs: How Bacteria Respond to Changing Environments",
            "We live in a microbial world. Bacteria inhabit most niches on earth, from the sea \
             floor to the high desert, and even in and on our bodies. For example, trillions of \
             bacteria live in your gut. This community of microorganisms is called the gut \
             microbiota, and it is a key indicator of good health. Discovering what makes these \
             gut bacteria special is one of the fastest growing areas of scientific research, \
             spanning areas of chemistry, biology, and methods in big data analysis. These \
             discoveries are leading to new ways to stop disease by controlling which bacteria \
             are in a person’s gut and what those bacteria do. The key to surviving in our gut, \
             as in any niche, is the ability of bacteria to adapt to changing conditions. Changes \
             in local conditions are considered stresses, from different food sources to nutrient \
             deprivation to exposure to harsh conditions like heat or acid or salt and others. \
             Bacteria have evolved methods of dealing with these changes through physiological \
             changes within the cell, known as stress responses.",
        ])),
        ..cosmos_default.clone()
    };
    let cosmos_ucsd = SummerCamp {
        location: Some("San Diego, CA"),
        identifier: "COSMOS UCSD",
        specialization: Some(make_specializations(&[
            "Cluster 1 - Computers in Everyday Life",
            "These days computers are everywhere, from our coffee makers and thermostats to our \
             cell phones and televisions. They make our cars safer and more efficient; they \
             perform advanced image processing in intelligent devices; they are the engines \
             behind creating our movies, television shows, and our video games; and they fuel the \
             Internet of Things. This course will focus on the basics of computing and coding, \
             making it accessible to students who have no prior programming experience. It \
             provides an introduction to computation through lectures, guest speakers, and \
             projects. It starts by teaching the fundamentals of programming in Python, one of \
             the most commonly used programming languages in the world, moves to C/C++ for \
             working with Arduino to build embedded systems, and the cluster concludes with \
             students creating a final project while working with teammates to refine, extend and \
             apply the computing skills of their choosing.",
            "Cluster 2 - Engineering Design and Control of Kinetic Sculptures",
            "Mechanical Engineering and Computer Control are brought together in many modern \
             products that have moving parts, ranging from heavy automobiles to light-weight \
             drones and robotic vacuum cleaners. In this cluster, students will analyze, design \
             and build Kinetic (Moving) Sculptures operated under Automatic Control to get a \
             comprehensive introduction to mixed disciplines in the field of engineering. \
             Students design and analyse a pendulum clock during the first week to become \
             familiar with Inventor, AutoCAD, running 2D dynamic simulations, and (remote) \
             manufacturing capabilities of a LASERcamm and a 3D printer. In the following weeks, \
             Mechanical Engineering methods will be used to analyse, design and build three \
             dimensional kinetic sculptures where marbles move along ramps, bounce on trampolines \
             and drop in baskets. The sculptures are augmented with sensors, motors and computer \
             control to emphasize the mix of engineering skills needed to design a reliable and \
             automatically controlled kinetic sculpture. The students attending this cluster will \
             walk away with valuable engineering experiences that include the use of modern \
             micro-processor controller to measure and analyze timing and mechanical behavior of \
             their design and integrating engineering design and control principles throughout \
             the curriculum of this cluster. Moreover, student will be able to (remotely) use the \
             state of the art facilities at the Mechanical and Aerospace Engineering (MAE) \
             department that include the MAE Design Studio, LASERcamm and 3D Printers for rapid \
             prototyping along with advanced computer laboratories for creating computer \
             drawings, running dynamic simulations and programming a microcontroller. Examples of \
             prior year projects can be seen here.",
            "Cluster 3 - Climate Change",
            "Climate Change is one of the most important and controversial issues facing our \
             world.  This cluster will break Climate Change into four parts.  The first section \
             will focus on the science of Green House Gases, GHGs, and their impact on the \
             atmospheric energy balance.  In the next section we will introduce the current \
             research conducted at UC San Diego examining the role of aerosols on the energy \
             balance and climate. These aerosols are influenced by the biology in the ocean and \
             are subsequent chemical transformation in gas phase reactions which serve as the \
             third section. The cluster will explore how global industrial human activity has \
             impacted health, food security, and land utilization.  We will also review how we \
             might mitigate climate change through reduced utilization, alternate energy sources, \
             carbon abatement and geoengineering.",
            "Cluster 4 - Structural Engineering: Building Better",
            "In Cluster 4, we like to build AND break things. We build small scale models of all \
             types of structures (bridges, buildings, foundations, soils, underground pipes, \
             aerospace structures, wind turbines, automobiles, human body, etc.) to see how we as \
             engineers can put together different components to build strong structural systems.  \
             Every crack and every snap is exciting!  We want to understand how and why it \
             failed, discuss what it means, and consider different methods of improving the \
             design to build it better!  To further the understanding of building materials, the \
             effects of natural forces such earthquakes, blasts and wind, and project planning \
             and building, we will also do a number of hands-on laboratories.  No matter what the \
             structure is, we want to learn to build it better.  We will introduce you to \
             structural engineering and immerse you in the design and problem solving process.",
            "Cluster 5 - Photonics: Light-based Technologies in Everyday Life",
            "We seldom realize how much our everyday life uses photonics, or light-based \
             technologies, such as in cell phone display, traffic light, DVD player, solar cells, \
             microscopes, endoscopes, optical fiber transmission, etc. The progress of photonics \
             is rapid, similar to Moore’s Law for electronics. One recent aspect of this progress \
             is the integration of photonics with electronics to produce high-speed Silicon \
             photonics integrated circuits. Other advances in photonics includes the integration \
             of artificial intelligence with computational optics and the development of \
             optically assisted diagnostics or therapeutic medical devices. While the economic \
             driver for the 20th century was electronics, the economic driver for the 21st \
             century is photonics.",
            "Cluster 6 - Biodiesel from Renewable Sources",
            "This course will introduce students to renewable biofuels. This is a laboratory \
             intensive experience where the students will extract and purify oil (lipids) from \
             biomass, convert the oil into Fatty Acid Methyl Esters, FAMEs, also known as \
             biodiesel, wash and purify the biodiesel, and then analyze the quality of the \
             finished product. They will use advanced instrumentation such as FTIR, GCMS, \
             Chromatography, and Bomb Calorimetry to determine the quality of their fuel.",
            "Cluster 7 - R4US - Robots for Undersea Science (Formerly known as H4O: Hacking for \
             Oceans)",
            "Open to students finishing their junior year prior to the summer. Desirable skills \
             (nice to have but not required): Biology, robotics (e.g., FIRST, VEX, others) basic \
             computer programming experience (Python and/or C/C++), hands-on engineering class, \
             and/or physics. Most importantly, “don’t be afraid of showing that you don’t know \
             something”, being willing to learn and not being afraid of making mistakes is key. \
             Please mention these skills and your attitude on learning in your application. ",
            "Cluster 8 - Tissue Engineering and Regenerative Medicine",
            "Tissue Engineering (TE) and Regenerative Medicine (RM) both seek to harness the \
             power of biology and chemistry with the precision of engineering to restore, \
             maintain, or improve tissue functions. TE seeks to do so through the application of \
             engineering and life sciences to develop biological substitutes, whereas RM targets \
             therapies to induce regeneration of cells, tissues, and organs. TE-RM are exciting \
             and interdisciplinary fields involving engineers, biologists, chemists, material \
             scientists, and doctors. TE-RM are increasingly providing alternative treatments for \
             medical conditions where traditional treatments such as drugs, medical devices, or \
             transplants have limitations. TE-RM products are rapidly evolving from potent \
             molecules and materials to induce regeneration, to isolated cells to reconstitute \
             damaged tissue, culture-expanded cells to repair damaged knees, modified cells to \
             combat cancer, and formed tissues for drug screening, for engineered skin to treat \
             wounds and burns, and for replacement tissues and organs.",
            "Cluster 9 - Music and Technology",
            "You do not have to be a musician to have fun and learn how science and engineering \
             can be used to create, compose, transform and perform sound and music. In Cluster 9 \
             you will learn about sound, music, and technology as we explore the many ways in \
             which technology is used to synthesize and analyze sounds and create music. Please \
             keep in mind that you do not need to be a musician or composer to enjoy Cluster 9, \
             you only need a creative and curious mind and a fascination for sound and music.",
            "Cluster 10 - Robot Inventors",
            "TARS, R2D2, WALL-E, .... these robots have captured all of our imaginations, but \
             reality isn't really that far off. Researchers around the world have made humanoid \
             robots to study how children learn, created robotic animals to carry supplies in war \
             zones, and built highly advanced surgical robots that act like snakes. Robotics is a \
             highly diverse field, bringing together skills of mechanical design, electronics, \
             artificial intelligence, programming, mechatronics, and even social sciences. This \
             cluster will introduce the students to this exciting field and will focus on \
             teaching students how to conceptualize, design, and create a custom robot. They will \
             learn about the mechanical design principles (CAD, basic prototyping), the sensors \
             through which robots interact with the world, and the programming that gives them \
             their autonomy. It is required that students have a basic background in programming \
             (any language, such as Java, C, Python, C++, JavaScript, etc ...). The emphasis for \
             this cluster is towards students with no prior exposure to robotics. The cluster \
             consists of lectures from the cluster faculty and guest speakers, a set of hands-on \
             projects, all leading to a final design challenge using hardware and software. \
             Students are highly encouraged to bring a PC laptop with them for this cluster.  If \
             students do not have a PC laptop available, one will be available to check out.",
            "Cluster 11 - Introduction to Autonomous Vehicles",
            "In this Cluster, we incorporate engineering theory and good practice in the \
             development of scale autonomous cars. We start by using a realistic robotics \
             simulator and an Artificial Intelligence framework (deep learning) on the student’s \
             own computer (see minimal requirements).  Then students will compete online using an \
             external simulator server against other students in the Cluster, and with parents' \
             permission, compete against competitors from around the world. While students learn \
             how to use the AI framework in the simulator, small teams will be designing and \
             building a physical scale robocar; mechanical and electrical fast prototyping will \
             be taught. Teams will apply what they have done in the simulator to their scale \
             autonomous car using GPU Accelerate Artificial Intelligence models. After the AI \
             deep learning portion of the class, students continue to develop autonomy with their \
             robots using traditional programming with Python and OpenCV.",
            "Cluster 12 - Machine Learning: Can We Teach a Computer to Think?",
            "Have you ever wondered how Alexa or Siri learned to converse with us, or how \
             ever-improving autopilot systems in self-driving cars are developed? The answer is \
             “machine learning,” an explosively-growing field in the last few years. Machine \
             learning is a form of artificial intelligence and it allows us to train computers \
             using the data we provide. For example if we provide a set of pictures of cats and \
             things that are not cats, a machine learning algorithm can teach the computer to \
             recognize cats. Therefore when the computer sees a new image later, it can tell \
             whether it is a cat or not by itself. Machine learning has permeated our daily lives \
             and is driving innovation in fields like medical diagnosis, face detection, \
             recommendation systems for shopping sites, automatic language translation and \
             climate study.",
            "Cluster 13 - Video Game Programming and Game AI Design",
            "Nearly every person on the planet who has access to technology has played a video \
             game.  The result is a video game industry that earns hundreds of billions of \
             dollars in revenue annually.  In this vibrant area of the economy, how do you design \
             video games that are appealing to players?  Moreover, how do you design the \
             Artificial Intelligence (AI) of the agents in the game, whether those be the \
             Non-Player Characters in the world or the AI that competes against the human player?  \
             In this cluster, students will learn how to answer these questions while designing \
             video games in Unity (C#).  Although the games they build may vary based on student \
             interest, they will likely build games like the story-driven classic Legend of \
             Zelda, Tower Defense games, and Real-Time Strategy games.  Each week, the students \
             will create a new game with the final project bringing together the overarching \
             concepts from the cluster.",
        ])),
        ..cosmos_default.clone()
    };
    let cosmos_ucdavis = SummerCamp {
        location: Some("Sacramento, CA"),
        identifier: "COSMOS UC Davis",
        specialization: Some(make_specializations(&[
            "Cluster 1 -- Quantum Mechanics and Applications to Nanotechnology",
            "Is it a particle? Is it a wave? It's both!  Electrons, normally considered \
             particles, can instead behave as waves when they are scattered by an ordered array \
             of atoms in a crystal. Similarly, the photoelectric effect can only be explained if \
             the electromagnetic waves which describe light behaves like particles called \
             photons. Quantum mechanics explains this dichotomy and thereby provides the \
             fundamental description of the perplexing fashion in which matter behaves at very \
             short distances.  Hence, quantum mechanics contains the principles needed to \
             understand fields from solid-state physics to electronics and biology by explaining \
             properties of atoms, chemical bonds, and how the periodic table of elements works. \
             In the first part of the cluster, students will learn some of the basic theoretical \
             principles and how to solve basic quantum mechanical problems computationally, \
             laying the foundation for interpreting the experiments in the second part of the \
             cluster.",
            "Cluster 2 -- Physics & Engineering: From the scientific method to technological \
             applications",
            "This section will explore the evolution of man's understanding of the rational \
             world, from the ancient Greeks to Newton's equations of motion. We illuminate how \
             simple observations can produce remarkable revelations of the world around us, and \
             how theoretical abstractions can be combined with observations to provide reliable \
             and important explanations and predictions that can be used in science and \
             technology. We will investigate the foundations of western scientific techniques in \
             physical science, explore how and why we have come to develop the \"scientific \
             method\" of proving and disproving hypotheses, what it means to do basic and applied \
             research, and how the principles of Newtonian mechanics, quantum mechanics, and \
             special relativity are directly linked to the centerpiece of the Cluster: The \
             relationships between observation/experiment, physics models, and technology.",
            "Cluster 3 --  Introduction to Engineering Mechanics",
            "Mechanical Engineering (2 Weeks) Modern Manufacturing Practice focuses on 3D \
             printing, assembly, and machine shop operations.  Understanding sustainable \
             manufacturing, including the economic, environmental, and social dimensions.  \
             Strength of Materials, including stresses and deformations in Internal Combustion \
             Engines (ICE) and Electric Cars. Turbomachinery and Wind Energy.  Systems and \
             Control, including sensors, actuators, and smart machinery. Feedback principles with \
             applications to vehicle dynamics . Aerospace Engineering (2 Weeks) What makes \
             Airplanes fly? Rocket Science and How Satellites Work will be covered, including the \
             physical, mathematical, and computational aspects of aerodynamics, space dynamics, \
             and propulsion at the high school level. Planned activities include using a water \
             table experiment to demonstrate wave patterns analogous to shock waves in supersonic \
             flows over airfoils or in nozzles and flying model airplanes and rockets. (Home \
             Experiments)",
            "Cluster 4 -- Introduction to Astrophysics",
            "This cluster consists of four interrelated core courses that will be taught \
             throughout the duration of the cluster.  The courses are: “Foundations of \
             Astronomy,” “Star and Planet Formation,” “Stellar Evolution,” and “Intro to \
             Cosmology.”  These courses are intended to provide students with a good background \
             in some of the most important aspects of astrophysics, and then to apply this \
             knowledge to some of the most interesting recent discoveries in the field.  There \
             will be special topic lectures from the instructors and guest lecturers.  In \
             addition, the students will work on research projects in astrophysics.  The research \
             projects will involve basic Python programming; while some familiarity with Python \
             or another programming language would be helpful, all needed programming skills will \
             be taught as part of the curriculum.",
            "Cluster 5 -- Computers in Biophysics & Robotics",
            "We know that everything, living or non-living, is made out of molecules. Molecules \
             are random walkers that keep bumping into each other and changing their trajectory, \
             shape and even their chemical identity. How, then, does a collection of such random \
             walkers assemble into incredibly organized and precise molecular machines that make \
             a living system function? In this course we will learn how to describe random \
             walkers using probability theory and primarily computer simulations. Students will \
             learn and use subsets of the C and Python programming languages to simulate the \
             behavior of biological molecules inside a living cell.",
            "Cluster 6 -- Mathematics",
            "This cluster is designed to introduce students with a strong interest in mathematics \
             to several advanced topics. These topics would ordinarily be studied at the advanced \
             undergraduate level, but all lend themselves to an introductory course at the high \
             school level. No prior experience in any of these topics is expected, but enthusiasm \
             for and interest in mathematics is essential. Students in this cluster will gain \
             insight into what to expect from studying mathematics, or applied mathematics, in \
             university. There will be an emphasis on doing mathematics and getting hands-on \
             experience!",
            "Cluster 7 -- Introduction to Plant Microbiology",
            "In this cluster, we will introduce students to the microbes that live on and in \
             plants. Students will learn to appreciate that all plants carry such microbes, that \
             these microbes represent a rich and diverse assemblage of microscopic shapes and \
             species, and that there are many ways in which these microbes may impact, for better \
             or worse, the health of their host plant. Students will be introduced to basic \
             concepts of microbiology, plant disease, food security, and food safety. Students \
             will learn simple laboratory techniques related to the fields of microbiology and \
             plant pathology. For their project, students will formulate and test a hypothesis, \
             isolate and describe microorganisms from plants, and present their findings in a \
             scientific poster and presentation. Students will also contribute to the development \
             and testing of a plant-microbe-inspired game.",
            "Cluster 8 -- From Semiconductors to Code: Exploring Analog and Digital Electronics",
            "This cluster is designed for aspiring students seeking to expand their knowledge and \
             hands-on experience in analog and digital electronics. Throughout the program, \
             participants will; Explore semiconductor physics and gain a foundational \
             understanding of the materials and principles that power modern electronic devices. \
             Learn to build analog circuits, including amplifiers, which are essential for many \
             real-world applications. Delve into digital electronics, study digital logic, \
             microcontrollers, and integrated circuits, and understand how they form the backbone \
             of contemporary computing and communication systems. Engage in hands-on projects \
             that bring theoretical concepts to life, culminating in the creation of functional \
             electronic prototypes, Experience an integrated learning approach and the synergy \
             between hardware and software that highlight the interdependence of these \
             disciplines.",
            "Cluster 9 -- Mathematical Modeling of Biological Systems",
            "This cluster will introduce students to a wide variety of mathematical models used \
             in biology. Students will learn how to construct mathematical models and use \
             mathematical techniques to analyze these models to gain insight into biological \
             phenomena. Mathematical topics covered include difference equations, differential \
             equations, probability, network theory, and game theory. Biological topics covered \
             range from ecology and epidemiology to physiology and cell biology. No prior \
             knowledge of these mathematical modeling methods or biological topics is necessary, \
             but a strong interest in mathematics and biology is essential. Computer programming \
             is integral to cluster activities. While no prior experience in programming is \
             required, a  willingness to learn is expected. In addition to the core courses \
             described below, this cluster will have weekly guest lectures by UC Davis faculty \
             working at the interface of mathematics and biology.",
            "Cluster 10 -- Sustainable Transportation",
            "A sustainable transportation system meets society’s needs for movement while \
             minimizing environmental harms, fostering healthy and equitable communities, and \
             supporting economic growth.  This cluster considers the science behind sustainable \
             transportation and examines sustainability in four areas:  system design, vehicle \
             technology, public transit, and micro-mobility. In all four courses, we will \
             consider the ways that science can inform policy.  Lectures and projects will draw \
             on multiple disciplines, including physics, mechanical engineering, civil \
             engineering, environmental engineering, economics, and statistics.",
            "Cluster 11 -- Beyond the Numbers: Exploring Data Science and Big Data Innovation",
            "Data is everywhere—from social media trends that shape our daily interactions to \
             scientific research that drives discoveries—and the ability to harness this data \
             responsibly is a skill of growing importance. This cluster offers courses crafted \
             for high school students who are eager to apply their mathematical knowledge to gain \
             insights into the world of data science. This field creatively combines statistical \
             methods, modern AI tools, computational techniques, critical thinking, and ethical \
             responsibility. In this cluster, students will learn to see data not just as static \
             numbers in a table, but as a powerful resource to solve problems, uncover trends, \
             make informed decisions, and drive innovation.",
            "Cluster 12 -- Introduction to Machine Learning",
            "Machine learning (ML), a subset of Artificial Intelligence, is considered a \
             disruptive technology with a wide range of applications in various domains, such as \
             healthcare, agriculture and the environment, robotics, and automation, among other \
             successful applications in our daily lives. In recent years, machine learning has \
             substantially helped various enterprises, researchers, and policymakers make \
             data-driven decisions based on historical observations and data.",
        ])),
        ..cosmos_default.clone()
    };
    let cosmos_ucsc = SummerCamp {
        location: Some("Santa Cruz, CA"),
        identifier: "COSMOS UCSC",
        specialization: Some(make_specializations(&[
            "Cluster 1 -- Linear Algebra and Discrete Math",
            "The main goal of our cluster is exploration. At this point in your education you’ve \
             likely seen the rudiments of mathematics (basic algebra, pre-calculus, maybe even \
             some Calculus, Linear Algebra, and Differential equations!) If all of mathematics \
             were a house (or maybe a Hilbert Hotel?), then I would humbly suggest that this is \
             akin to just having come through the door and having only seen the foyer. These \
             courses will be a nice survey of some of the rooms in this house that are familiar \
             to what you have already learned and some that are brand new with new tools and new \
             ways of thinking! By the end of our journey, while we can not show you every room in \
             the house, we hope to leave you with an appreciation of how much math there really \
             is, and be inspired to search out more rooms (or even build some yourself!)",
            "Cluster 2: Nanochemistry and Nanotechnology",
            "At the nanometer scale (10(raised to the -9th power)m), the chemical and physical \
             properties of materials and structures show drastic deviations from those of their \
             atomic or bulk forms. Exploiting these new properties has sparked research in \
             energy, electronics, and a diversity of areas. We will introduce basic principles of \
             nanoscience and nanotechnology, and demonstrate the applications of functional \
             nanomaterials through in-house demonstrations and hands-on experiments. Students \
             will learn about the preparation and functionalization of nanomaterials, their \
             controlled assemblies, and the potential impacts on future technologies. Students \
             will also visit research facilities and laboratories as a foundation for \
             understanding, evaluating, and explaining nanotechnology. In this cluster, we will \
             uncover patterns that transcend specific technologies, enabling us to evaluate \
             whatever we create in the future.",
            "Cluster 3 -- Making an Animal: Development and Bioinformatics",
            " In this cluster, students will learn the principles of animal development and how \
             they are related to human diseases.  The students will also get hands-on training on \
             bioinformatics for running single cell analyses.",
            "Cluster 4 -- Quantum Information Science and Engineering (QISE)",
            "We are currently in the midst of the second quantum revolution, a major paradigm \
             shift that aims to fundamentally change the methods of encoding and processing \
             information.  This exciting and emerging effort, which occurs at the intersection of \
             physics, chemistry, engineering, and materials science constitutes the field of \
             Quantum Information Science and Engineering (QISE). This effort may lead to the \
             creation of quantum computation devices, which will be exponentially faster than \
             currently existing computers, and means of quantum communication that will be \
             completely protected from hacking and eavesdropping. The methods of QISE are also \
             broadly used for describing a broad variety of devices that display quantum \
             properties, including but not limited to superconductive electric circuits, \
             single-electron transistors and microscale-size crystals. In this COMOS cluster we \
             will introduce the basic properties of QISE and demonstrate emergent applications \
             from this field of research through lectures, invited talks from experts, field \
             trips, and in-class activities.",
            "Cluster 5 -- Video Games: The Design of Fun from Concept to Code",
            "The goal of this cluster is to introduce high school students to computer science \
             and the design principles used in creating a computer game. Computer games are no \
             longer just for young children and enthusiasts. Modern games show a wide variety of \
             features, from the rich graphics and detailed rule sets of World of Warcraft, to the \
             simpler, more casual gameplay of Bejeweled, which appeal to a wide variety of \
             audiences. In this cluster we focus on both the technical and design sides of \
             creating computer games, through analysis of popular existing games and a series of \
             projects in which students will build their own games. Students will learn the \
             design principles for creating games that are fun, engaging, and interactive, and \
             how to understand and build these complex software systems.",
            "Cluster 6 -- Introduction to Smart and Sustainable Power",
            "This course aims to provide a breadth of knowledge of power systems, smart grids, as \
             well as renewable energy sources and storage. Students will understand the relation \
             between power grids and the internet. Various renewables such as solar, wind, \
             hydropower, geothermal, and tidal energy will be introduced. Solar- and wind-site \
             assessment, electric vehicles, and sustainable microgrids will be discussed. \
             Important concepts of electrical machinery, power electronics, and industrial \
             internet of things will also be described. Finally, the latest research topics on \
             grid resilience under extreme events (e.g., wildfires and earthquakes), electrical \
             faults, and energy data analytics will be presented. There will be a few hands-on \
             lab sessions for facilitating students’ understanding of electricity fundamentals.",
            "Cluster 7 -- Shining a Light on the Future: The Photonics Revolution in Healthcare, \
             Energy, and Information Technologies",
            "In the last half-century, photonics, much like semiconductors, has initiated a \
             groundbreaking shift in various sectors, particularly in healthcare, energy, and \
             information technologies. Photonics, the science of light generation, manipulation, \
             and detection, has become an integral part of our daily lives and the backbone of \
             numerous modern technologies. This cluster aims to explore the fundamental \
             principles of photonics and its diverse applications across critical fields. From \
             advancements in medical imaging and diagnostics in healthcare, efficient energy \
             sources in solar power technology, to high-speed optical communication systems in \
             IT, photonics is at the forefront of innovation. Participants will delve into how \
             photonics is driving the future of these sectors, understanding the underlying \
             technologies such as fiber optics, biophotonics, laser engineering, and \
             communication systems. This cluster is ideal for students and professionals eager to \
             understand the role of photonics in shaping future technologies and its application \
             in improving healthcare, enhancing renewable energy, and evolving information \
             technology.",
            "Cluster 8 -- Practical Applications of Chemical Principles",
            "Cluster 8 focuses on practical applications of fundamental chemical principles. Both \
             courses of the cluster will focus on providing a more in-depth coverage of key \
             chemical and physical concepts that students will have previously received and then \
             use that knowledge to understand how those principles are at play in everyday \
             phenomena and the functional materials that make direct use of these principles. \
             Participation in the cluster will involve lectures, hands-on experience, and \
             (time-permitting) field trips.",
            "Cluster 9 -- Molecular Biology Investigations",
            "In this cluster, students will learn how molecular biology research and \
             investigations contributes to our understanding of biology. Students will also \
             acquire hands-on wet lab experience in genetics and/or molecular cloning, as well as \
             dry lab skills such as basic bioinformatics, critical scientific reading, and paper \
             writing. Students who engage in our cluster will be well prepared for introductory \
             molecular biology laboratory coursework in their future. Possible field trips to the \
             SLAC National Accelerator Lab and UCSC botanic garden will provide outdoor learning \
             and social opportunities.",
            "Cluster 10 -- Semiconductor Materials and Device Engineering",
            "In the past 50 years, semiconductors have changed our lives in a way that has never \
             occurred before in human history. Every aspect of our modern lives involves \
             semiconductors. This cluster will cover the basics of semiconductors, how to make \
             semiconductors, and how basic semiconductor devices work. This cluster covers the \
             device-level operation (i.e., LED, lasers, transistor, solar cell, memory device, \
             etc.), so students interested in learning more about how semiconductors work and how \
             to make them for useful engineering applications, will find this cluster interesting.",
            "Cluster 11 -- Brain-Inspired Machine Learning",
            "This course explores how to make machine learning algorithms as efficient as the \
             brain. Topics to be covered include logistic and linear regression, neural networks, \
             deep learning, computer vision with convolutional neural networks, sequence-based \
             models, transformers, low-power machine learning techniques, low-precision and \
             compressed models, and spiking neural networks.",
            "Cluster 12 -- Exploring the Structure of Quantum Materials by Neutron Scattering and \
             X-ray Diffraction",
            "This cluster focuses on the scattering interaction between matter with matter and \
             light with matter as well as their use in science. The most common techniques \
             include X-ray and neutron scattering and diffraction. This topic is covered broadly \
             from the fundamental mathematical theory to the experimental applications performed \
             by scientists to the effects seen in everyday life. The importance of scattering and \
             diffraction in the fields of physics and chemistry will be emphasized. The syllabus \
             will include lectures, labs, field trips (e.g. SSRL), etc.",
            "Cluster 13 -- Safe and Secure Autonomous Systems",
            "This cluster will introduce students to the basic principles of safety and security \
             and their role in developing trustworthy artificial intelligence (AI) agents for \
             autonomous decision making for self-driving vehicles and computer networks",
        ])),
        ..cosmos_default.clone()
    };
    let cosmos_ucla = SummerCamp {
        location: Some("Los Angeles, CA"),
        identifier: "COSMOS UCLA",
        specialization: Some(make_specializations(&[
            "Cluster 1: Brain-Inspired Computing: Learning in Biological; Artificial Neural Networks",
            "The world is entering a new era of Artificial intelligence (AI) technology in which machines will become increasingly capable of performing “human” tasks such as speaking a language, driving an automobile, writing a computer program, or recognizing images in photographs and videos. To perform such tasks, AI systems rely on artificial neural networks that learn from their own experiences in the much same way that humans and animals do. Indeed, AI technology has arisen largely from efforts by scientists and engineers to build machines that mimic the brain. For this reason, modern AI is sometimes referred to as “brain-inspired computing.” ",
            "Cluster 2: Ecosystem Responses to Climate from Plants to Planet: This cluster analyzes and presents data from space and lab sensors",
            "In this cluster, we will introduce students to the ecosystems of the globe, and the plants that dominate them and determine their function, including contributions to the carbon and water cycles and atmospheric CO2. We will examine the physiology of the plants in real time in the lab and urban forest, and analyze their functions in the ecosystems globally and locally using satellite imagery. Students will learn to appreciate the diversity of plants, their contribution to our local and planetary environment, and how they interact with the geosphere to determine climate. Additionally, students will learn how climate change is influenced by the plants, which in turn are affected by climate. Students will learn both laboratory techniques in plant physiology, as well as urban ecology methods, and the analysis of remote sensing data. For their project, students will formulate and test a hypothesis for the interactions of plants and ecosystems with their environment in the city of Los Angeles, or globally for planet Earth, and present their findings in a scientific poster and presentation. Students will also contribute to the development and testing of a plant-ecosystem-global climate-inspired game.",
            "Cluster 3: Exploring the Evolution of Animal Form: From Fossils to Embryos",
            "Charles Darwin ended his Origin of the Species with “…endless forms most beautiful and most wonderful have been, and are being, evolved.” While he got this mostly right, he missed the mark on one key word – “endless.” Even a cursory look around the natural world reveals that the diversity of animal form, while arguably beautiful and wonderful, is not endless. Some animal forms have evolved repeatedly (e.g., fewer than five fingers or toes in vertebrates), while others have never evolved (e.g., more than four limbs in vertebrates). Why this is the case remains one of the largest outstanding questions in modern biology, and will be the focus of this module.",
            "Cluster 4: High Success: Hydrogen Is Green Headway to SUstainability, Carbon Capture, Energy-transition, and SuStainizability",
            "HIGH SUCCESS is a cluster that focuses on issues that stand at the top of societal agendas and particularly concerns of teens. These include, Education, Environment, Economy, and Climate Change, https://ed.stanford.edu/news/what-do-teens-care-about-stanford-education-researchers-uncover-top-concerns-voiced-letters, which as will be shown in this cluster are related to science and engineering concepts focused on Hydrogen, Sustainability, Carbon Capture, and the Energy transition.",
            "Cluster 5: From Self-Balancing Mini Robot Cars to Rockets: Exploring Mechanical and Aerospace Engineering",
            "In this cluster, the students will explore mechanical and aerospace engineering by using mechatronics as integral learning tools. Mechatronics, which is a combination of mechanics and electronics, is everywhere, from toasters to sophisticated robots.  Fueled in part by continual advances in computing software and hardware, mechatronic devices make our lives more convenient, safer, and more efficient.  Mechatronic devices are also essential in performing modern engineering experiments necessary to develop, test, and validate engineering models and designs.",
            "Cluster 6: From Medicine to Hollywood: Artificial Intelligence for Speech and Imagery",
            "Explore the world of artificial intelligence (AI) for images and sounds. Develop your own AI algorithms where you can hear and see the results. Speech and Image processing involves analyzing, synthesizing, and recognizing soundwaves and images. In this cluster, you will learn about how humans and machines produce and perceive speech like Siri does and recognize objects in images like emerging self-driving cars do. The first two weeks will focus on speech processing and the last two weeks on image processing. In particular, the instructors will show the common synergies between processing speech (a 1-dimensional signal) and images (a 2-dimensional signal). The instructors do not assume familiarity with Python, but recommend “basic exposure” to programming in any language, where “basic exposure” is defined in the pre-requisites of this document. Additional guest speakers who are industry and faculty practitioners will introduce cutting edge technical topics under the same prerequisite umbrella, to ensure a high-school appropriate delivery of content from all cluster material. Students will gain hands-on experience through the “Computer Lab”, where projects are conducted on real data from various applications: in particular, (a) medicine; and (b) entertainment, inspired by the local connections at UCLA Hospital and Hollywood, respectively.",
            "Cluster 7: Revealing Molecular Structure",
            "Chemical Principles Applied to Chemical Structure Before covering analytical and characterization techniques used for structure determination, it is necessary to understand relevant chemical principles. Topics such as atomic structure, introductory quantum theory, chemical bonding and structures of inorganic and of organic compounds, as well as properties of acids and bases will be covered. Laboratory periods will include both dry- and wet-lab activities.  Analytical and Characterization Methods for Structure Determination Students will be introduced to common analytical and characterization techniques used for structure determination. The curriculum from Chemical Principles Applied to Chemical Structure will be heavily built upon. Acid-base titration, colorimetry, infrared spectroscopy, mass spectrometry, nuclear magnetic resonance spectroscopy and X-ray crystallography will be covered. Compounds will either be provided to or synthesized by students.",
            "Cluster 8: Bit by Bit: Mathematics and Technologies of the Information Age",
            "How does NASA communicate with the Voyager space probe over 15 billion miles from Earth? How does this relate to QR codes and .zip files? How do Wi-Fi and 5G deliver real-time video using wireless signals? In this cluster, we will explore how information is stored, protected, and reliably communicated in today’s increasingly digital world. Whether you are sending a text, streaming a video, or making an online purchase, information is constantly being transported from one point to another. How do we make sure that this information does not get lost, corrupted, or intercepted along the way? You will learn how special techniques are used to protect privacy, compress data, and detect and even fix errors when storing and communicating information. Then, you will learn how a transmitter encodes this data onto a wireless signal and how a receiver can reliably extract this data from that signal. This cluster will provide you with a deeper understanding of the mathematics and technologies underpinning the ongoing Information Age through hands-on learning, field trips around Los Angeles, the implementation of an end-to-end wireless communication system using software-defined radios.",
        ])),
        ..cosmos_default.clone()
    };
    [
        cosmos_uci,
        cosmos_ucsd,
        cosmos_ucdavis,
        cosmos_ucsc,
        cosmos_ucla,
    ]
    .into_iter()
}
