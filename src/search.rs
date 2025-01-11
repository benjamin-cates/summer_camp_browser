use crate::structs::SummerCamp;

fn count_matches(needle: &str, haystack: &str) -> usize {
    haystack.split(needle).count() - 1
}

impl SummerCamp {
    fn matcher_text(&self) -> Vec<(usize, String)> {
        let mut out = vec![];
        self.description.iter().for_each(|desc| {
            out.push((5, desc.to_lowercase()));
        });
        out.push((40, self.identifier.to_lowercase()));
        if let Some(loc) = self.location {
            out.push((40, loc.to_lowercase()));
        }
        if let Some(org) = self.organization {
            out.push((40, org.to_lowercase()));
        }
        if let Some(ref spec) = self.specialization {
            spec.iter().for_each(|(str, camp)| {
                out.push((10, str.to_lowercase()));
                out.extend(camp.matcher_text().into_iter());
            })
        }
        out
    }
    fn get_whole_match_score(matcher_text: &Vec<(usize, String)>, query: &str) -> usize {
        if query.len() < 2 {
            return 0;
        }
        matcher_text.iter().map(|(power, str)| power * count_matches(query, str)).sum()
    }
    pub fn get_match_score(&self, query: &str) -> usize {
        let matcher_text = self.matcher_text();
        let query = query.to_lowercase();
        let mut score = SummerCamp::get_whole_match_score(&matcher_text, query.as_str()) * 8;
        for word in query.split(" ") {
            if word.len() < 2 {
                continue;
            }
            score += SummerCamp::get_whole_match_score(&matcher_text, word);
        }
        score
    }
}
