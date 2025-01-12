use crate::structs::{Limit, Requirement, SummerCamp};

pub fn uci_brain_camp() -> SummerCamp {
    SummerCamp {
        acceptance_rate: None,
        link: Some("https://cnlm.uci.edu/braincamp/"),
        description: vec![
            "UCI Brain Camp is a summer program for middle and high school students that offers \
             an opportunity for immersion in neuroscience. The 2-week full-day camp (9am-4pm) \
             will feature lectures and seminars, workshops, laboratory tours, dissections and \
             other hands-on activities and experiments.",
            "Explore topics such as: Brain and spinal cord anatomy, Sensation and perception, \
             Learning and memory, Brain-machine interface, Neuropathology, Neuropharmacology, \
             Comparative neuroanatomy",
            "Participate in hands-on laboratory work to explore the latest tools, techniques and \
             technologies that are used in neuroscience laboratories at UC Irvine including: \
             Electrophysiology, Brain Imaging, Animal models, Optogenetics, DREADDs",
            "Participate in workshops on topics such as: Experimental design, Data analysis, \
             Scientific Communication and public speaking, Neuroethics",
            "Interact with world-renowned neuroscience professors, PhD students and \
             undergraduates to gain insight into college life and the possible careers in \
             neuroscience. Develop the tools and skills necessary for success in college \
             coursework.",
            "During the second week of the program, students will design experiments, analyze \
             data and present their work to their families as well as to an audience of UC Irvine \
             neuroscience faculty and students.",
        ],
        deadline: Some("unknown"),
        identifier: "UCI Brain Camp",
        length_wk: Some(4.0),
        application_opens: Some("12/1/24"),
        requirements: vec![
            Requirement::GradeRange(6..=12),
            Requirement::RequiredEssay("Unknown", Limit::Unspecified),
            Requirement::RecommendationForm("unknown"),
            Requirement::UnofficialTranscript,
        ],
        application_fee: Some(0.0),
        tuition: Some(2195.0),
        apply_link: None,
        last_updated: Some("1/11/25"),
        location: Some("Irvine, California"),
        organization: Some("University of California, Irvine"),
        ..Default::default()
    }
}

pub fn uci_ethics() -> SummerCamp {
    SummerCamp {
        link: Some("https://www.ethicscenter.uci.edu/awards/summersinternship.php"),
        description: vec![
            "Each summer, the UCI Ethics Center selects a few promising students for a mentoring \
             program. We plan a full online program again in 2024, open to all qualified college, \
             graduate, and high school students worldwide.",
            "Students will be allowed to participate in only one module but may register on a \
             waitlist for a module that is over-subscribed. We will try to assign each student to \
             their preferred module. We will add extra modules if demand requires it in order to \
             keep the numbers below 30 for each module.",
        ],
        deadline: Some("unknown"),
        identifier: "UCI Ethics Center Summer Mentorship",
        length_wk: Some(4.0),
        requirements: vec![
            Requirement::GradeRange(9..=12),
            Requirement::RequiredEssay("Unknown", Limit::Unspecified),
        ],
        application_fee: Some(250.0),
        apply_link: None,
        last_updated: Some("1/11/25"),
        location: Some("Online"),
        organization: Some("Center for the Scientific Study of Ethics and Morality"),
        ..Default::default()
    }
}
