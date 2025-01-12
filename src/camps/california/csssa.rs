use crate::structs::{Requirement, SummerCamp};

pub(crate) fn get_csssa() -> SummerCamp {
    SummerCamp {
        link: Some("https://www.csssa.ca.gov/"),
        deadline: Some("2/28/25"),
        acceptance_rate: Some(20.0),
        tuition: Some(4830.0),
        application_fee: Some(20.0),
        apply_link: Some("https://csssa.embark.com/apply/dashboard"),
        location: Some("Santa Clarita, CA"),
        notes: vec!["GPA is not considered"],
        identifier: "California State Summer School for the Arts (CSSSA)",
        last_updated: Some("1/8/25"),
        organization: Some("CalArts"),
        length_wk: Some(4.0),
        requirements: vec![
            Requirement::UnofficialTranscript,
            Requirement::Custom("Art portfolio (depending on track)"),
            Requirement::GradeRange(9..=12),
        ],
        specialization: Some(vec![
            (
                "Animation",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/animation/animation-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["The Animation Program provides students with a unique opportunity to experience a broad range of possibilities within the world of animation. From fine art to digital animation, this program encourages students to explore their potentials, and bring movement to their artwork through an intense and rigorous curriculum. CSSSA Animation students learn diverse experimental and traditional animation techniques under the guidance of leading artists in the field. They study the history and work of animators from all over the world, they participate in figure drawing classes, and they learn from professional animation artists. Animation students at the summer program are dedicated, hardworking, and open to new ideas."],
                    ..Default::default()
                },
            ),
            (
                "Dance",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/dance/dance-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["CSSSA offers an exciting, pre-professional, collegiate-level Dance Program designed for students who are interested in pursuing a college degree and/or a professional career in dance. All dancers who apply to the program should have prior experience of at least two (2) years of technical training in ballet or modern dance. The Dance Program is designed to give students the tools and education they need to explore different styles of dance technique. Students are also provided with a stimulating environment in which to begin the creative choreographic process. All dancers are required to participate in mandatory ballet and modern technique classes as well as improvisation and composition classes daily. Repertory workshops and seminars are included, as well as master classes and rehearsals, with guest artists round out this powerful four-week curriculum."],
                    ..Default::default()
                },
            ),
            (
                "Film",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/film/film-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["The CSSSA Film Program embraces an auteur-driven philosophy, equipping students with the tools to articulate their unique cinematic voice. Participants will delve into a carefully curated selection of auteur films and benefit from masterclasses led by renowned industry professionals. This intensive program is designed for ambitious, self-motivated students eager to push creative boundaries while honing their critical and artistic perspectives."],
                    ..Default::default()
                },
            ),
            (
                "Music",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/music/music-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["Every summer the California State Summer School for the Arts welcomes 70 to 80 music students for exploration in the performing arts. These students come to CSSSA from all over California to develop their skills at the highest level. They engage in challenging developmental programs in the company of peers who share their exceptional abilities and love for the arts. The CSSSA Music Program is increasingly relevant in a time when educational programs in the arts are vanishing from our cultural landscape. The program is at the forefront of the exploration of an array of musical genre that reflects our multi-cultural society. The unique music curriculum at CSSSA features instruction in theory, music history, African Drumming and Dance, Indonesian Gamelan, individual instruction, and collaborative efforts with other CSSSA departments such as dance, theater, and creative writing. A wide range of professional musical artists and entrepreneurs visits our campus to perform and discuss their art, experiences and vision with our students. In addition, there are many performance opportunities using the great spaces at the California Institute of the Arts."],
                    ..Default::default()
                },
            ),
            (
                "Theater",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/theater/theater-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["The California State Summer School for the Arts Theater Program offers intensive professionally-oriented acting training for students who are serious about exploring the craft of acting and the possibilities of the art of theater in the modern world. An open mind, an adventuresome spirit, the desire and ability to work hard and enjoy it, the capacity to be a part of an ensemble, and a passion and willingness to explore a variety of approaches to acting are more important prerequisites for this program than extensive prior theater experience."],
                    ..Default::default()
                },
            ),
            (
                "Writing",
                SummerCamp {
                    #[rustfmt::skip]
                    requirements: vec![Requirement::Custom("Art portfolio submission: https://www.csssa.ca.gov/academic-programs/writing/writing-requirements/")],
                    #[rustfmt::skip]
                    description: vec!["The CSSSA Writing Program offers personalized and interactive workshops for approximately seventy talented and motivated young writers. A faculty of four professional writers and educators guide and instruct students in the techniques of fiction, poetry, non-fiction, and dramatic writing. We’re looking for students who have the courage to be themselves on the page. We’re looking for students who love language. We’re looking for students who want to tell their own stories: stories from their imagination, from their neighborhood, from their family. Ideally our students have begun to outgrow the High School reading list. They have struck out and found the writers that speak to their own inner life and experiences. Whether it’s edgy YA, slam poets at the local cafe or on YouTube, European novelists, or the latest New York playwrights, CSSSA students tend to have a list of writers they love."],
                    ..Default::default()
                },
            ),
        ]),
        ..Default::default()
    }
}
