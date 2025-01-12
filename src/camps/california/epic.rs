use crate::structs::{Limit, Requirement, SummerCamp};

use super::super::generators::make_specializations;

pub(crate) fn get_epic() -> SummerCamp {
    SummerCamp {
        application_opens: Some("1/6/25"),
        identifier: "Cal Poly EPIC",
        link: Some("https://epic.calpoly.edu/"),
        #[rustfmt::skip]
        apply_link: Some(
            "https://forms.office.com/pages/responsepage.aspx?id=2wING578lUSVNx03nMoq533E2_BjHYlEttFwI7cqoo1URUdONklXMVg4UzAzMlI3TE9JNFVDSUlQNC4u&route=shorturl",
        ),
        deadline: Some("2/7/25"),
        description: vec![
            "EPIC- Engineering Possibilities in College - is a one-week summer camp at Cal Poly \
             San Luis Obispo that introduces campers to the varied fields of engineering through \
             hands-on labs taught by university professors and industry professionals.  EPIC \
             campers see \"Learn by Doing\" in action when Cal Poly students show them the \
             satellites, prosthetic hands, race cars, concrete canoes, and other products they've \
             developed.  In addition, EPIC campers get to design, build, and test their own \
             projects!",
        ],
        requirements: vec![
            Requirement::GradeRange(7..=12),
            Requirement::RequiredEssay("Why would you like to attend EPIC?", Limit::Words(0..=250)),
            Requirement::Custom("No transcript or letter of rec accepted"),
        ],
        location: Some("San Luis Obispo, California"),
        organization: Some("Cal Poly San Luis Obispo"),
        last_updated: Some("1/7/25"),
        tuition: Some(2000.0),
        length_wk: Some(1.0),
        application_fee: Some(0.0),
        specialization: Some(make_specializations(&[
            "Aerospace Engineering",
            "Aerospace engineering prepares you to create more efficient and economical solutions \
             for aircraft, spacecraft and related systems and equipment such as drones and \
             satellites.",
            "Agricultural Engineering",
            "Agricultural Engineering is the field of study and application of engineering \
             science and designs principles for agriculture purposes, combining the various \
             disciplines of mechanical, civil, electrical, food science, environmental, software, \
             and chemical engineering to improve the efficiency of farms and agribusiness \
             enterprise as well as to ensure sustainability of natural and renewable resources.",
            "Architectural Engineering",
            "Architectural Engineering is the same as structural engineering but with \
             considerations given to the architecture (how the building looks on the outside, how \
             it will be used, ect.) and is a major unique to Cal Poly in the department of \
             Architecture and Environmental Design",
            "Biomedical Engineering",
            " Biomedical engineers use engineering principles and a knowledge of biological \
             sciences to design and create new technology to improve the delivery of health care.",
            "Bioresource Engineering",
            "Bioresource Engineering is an interdisciplinary program that integrates engineering, \
             design and the biological sciences. It is a unique profession that applies \
             engineering principles to the enhancement and sustainability of the world’s natural \
             resources. Bioresource Engineers seek solutions to problems that involve plants, \
             animals and the environment.",
            "Civil Engineering",
            "Civil Engineering is the design of systems used in civilization including \
             structures, transportation, water, and the interaction of these systems with the \
             ground (geotechnical)",
            "Computer Engineering",
            "Computer engineering, on the other hand, is a field at the intersection of \
             electrical engineering and computer science. Computer engineers research how to \
             build all varieties of computing systems from smartphones to integrated circuits.",
            "Computer Science",
            "Computer science focuses on the theory of computation. This includes writing code \
             that integrates data, data structures, algorithms, statistical models and more in \
             the most efficient way possible. This field also focuses on how to create algorithms \
             that efficiently achieve complex tasks, whether that task is emulating a human brain \
             or determining the best route for your Uber pool.",
            "Electrical Engineering",
            "Electrical engineers design, develop and build a wide range of electrical equipment \
             including computers, electronic devices, communication systems, test equipment, \
             electric power networks and more.",
            "Environmental Engineering",
            "Environmental engineering is the branch of engineering that is concerned with \
             protecting people from the effects of adverse environmental effects, such as \
             pollution, as well as improving environmental quality. Environmental engineers work \
             to improve recycling, waste disposal, public health, and water and air pollution \
             control.",
            "General Engineering",
            "\"General\" engineering students have a specific major path in mind and create a \
             customized program. Potential careers include: Audio engineering, Chemical \
             engineering, Technical sales, and Medicine.",
            "Industrial Engineering",
            "Industrial engineering profession that applies engineering analysis and methods to \
             the production of all manufactured goods and services.",
            "Liberal Arts Engineering",
            " Liberal Arts Engineering students obtain a wide range of innovative careers in \
             emerging professional fields that combine skills and interests in the technology, \
             arts, and culture. They are highly successful in obtaining technical, design, and \
             management careers in companies such as Warner Brothers Studios, Disney, Tesla, \
             Apple, DTS, and Universal Studios",
            "Manufacturing Engineering",
            "Manufacturing engineer plans, develops, and optimizes the processes of production \
             and designs products to be manufactured.",
            "Materials Engineering",
            "A degree in materials engineering will teach you what materials should be used to \
             create products and how to develop new materials to fit new and specific purposes.",
            "Mechanical Engineering",
            "Mechanical engineering is one of the broadest engineering disciplines. Mechanical \
             engineers design, develop, build, and test. They deal with anything that moves, from \
             components to machines to the human body.",
            "Software Engineering",
            "Software engineering students get a strong technical preparation in computer science \
             fundamentals. The program emphasizes a combination of technical and team management \
             skills. A graduate of the program is expected to understand the challenges of \
             large-scale systems development and to be equipped with the necessary technical, \
             process and people skills to be productive in a team environment.",
        ])),
        ..Default::default()
    }
}
