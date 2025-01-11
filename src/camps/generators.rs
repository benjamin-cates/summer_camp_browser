use crate::structs::SummerCamp;



pub fn make_specializations(tracks: &[&'static str]) -> Vec<(&'static str, SummerCamp)> {
    tracks
        .chunks(2)
        .map(|v| {
            (
                v[0],
                SummerCamp {
                    description: vec![v[1]],
                    ..Default::default()
                },
            )
        })
        .collect()
}
