use std::{fmt::Display, ops::RangeInclusive};

use serde::{Deserialize, Serialize};

pub struct Org {
    identifier: &'static str,
    web_page: &'static str,
    formal_name: &'static str,
    email: &'static str,
    logo: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SummerCamp {
    /// Name
    pub identifier: &'static str,
    /// Link to the website
    pub link: Option<&'static str>,
    /// Short description of the camp
    pub description: Vec<&'static str>,
    /// Which organization runs this camp?
    pub organization: Option<&'static str>,
    /// Number of weeks the camp lasts
    pub length_wk: Option<f64>,
    /// Location of the camp
    pub location: Option<&'static str>,

    /// Cost of attendance
    pub tuition: Option<f64>,
    /// Cost to submit application
    pub application_fee: Option<f64>,

    /// List of required application materials
    pub requirements: Vec<Requirement>,
    /// Deadline for the application
    pub deadline: Option<&'static str>,
    /// When the application opens
    pub application_opens: Option<&'static str>,
    /// Link to apply
    pub apply_link: Option<&'static str>,
    /// Can be thought of as "subcamps"
    pub specialization: Option<Vec<(&'static str, SummerCamp)>>,
    /// Acceptance rate out of 100%
    pub acceptance_rate: Option<f64>,

    /// Last time this entry was updated
    pub last_updated: Option<&'static str>,
    /// Additional notes
    pub notes: Vec<&'static str>,
    /// Keywords
    pub keywords: Vec<(&'static str, usize)>,
}

impl Default for SummerCamp {
    fn default() -> Self {
        Self {
            link: None,
            application_opens: None,
            apply_link: None,
            deadline: None,
            description: vec![],
            requirements: vec![],
            tuition: None,
            application_fee: None,
            identifier: "",
            last_updated: None,
            location: None,
            notes: vec![],
            organization: None,
            specialization: None,
            length_wk: None,
            acceptance_rate: None,
            keywords: vec![],
        }
    }
}
impl SummerCamp {
    pub fn with_defaults(self, other: &Self) -> Self {
        Self {
            link: self.link.or(other.link),
            deadline: self.deadline.or(other.deadline),
            identifier: self.identifier,
            organization: self.organization.or(other.organization),
            location: self.location.or(other.location),
            tuition: self.tuition.or(other.tuition),
            application_fee: self.application_fee.or(other.application_fee),
            requirements: [self.requirements, other.requirements.clone()].concat(),
            notes: [self.notes, other.notes.clone()].concat(),
            description: [self.description, other.description.clone()].concat(),
            application_opens: self.application_opens.or(other.application_opens),
            apply_link: self.apply_link.or(other.apply_link),
            last_updated: self.last_updated.or(other.last_updated),
            specialization: self.specialization.or_else(|| other.specialization.clone()),
            length_wk: self.length_wk.or(other.length_wk),
            acceptance_rate: self.acceptance_rate.or(other.acceptance_rate),
            keywords: [self.keywords, other.keywords.clone()].concat(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "type", content = "num")]
pub enum Limit {
    Words(RangeInclusive<usize>),
    Characters(RangeInclusive<usize>),
    Unspecified,
}
impl Default for Limit {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Requirement {
    OptionalEssay(&'static str, Limit),
    RequiredEssay(&'static str, Limit),
    ActivityList(&'static str),
    UnofficialTranscript,
    OfficialTranscript,
    CounselorRecommendation,
    LetterOfRec(&'static str),
    RecommendationForm(&'static str),
    AgeRange(RangeInclusive<usize>),
    GradeRange(RangeInclusive<usize>),
    SocialSecurityNumber,
    MinWeightedGPA(f64),
    MinUnweightedGPA(f64),
    TestScores(&'static str),
    Custom(&'static str),
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Words(range) => f.write_fmt(format_args!("{}-{} words", range.start(), range.end())),
            Self::Characters(range) => f.write_fmt(format_args!("{}-{} characters", range.start(), range.end())),
            Self::Unspecified => f.write_str("unspecified word count"),
        }
    }
}

impl Display for Requirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirement::OptionalEssay(str, limit) => {
                f.write_fmt(format_args!("Optional essay: {} ({})", str, limit))
            }
            Requirement::RequiredEssay(str, limit) => {
                f.write_fmt(format_args!("Essay: {} ({})", str, limit))
            }
            Requirement::ActivityList(str) => {
                f.write_fmt(format_args!("Activity list: {}", str))
            }
            Requirement::AgeRange(range) => {
                f.write_fmt(format_args!("Age eligibility: {}-{}", range.start(), range.end()))
            }
            Requirement::GradeRange(range) => {
                if *range.end() == 13 {
                    f.write_fmt(format_args!("Entering grades: {}-12 or recent high school grads", range.start()))
                }
                else {
                    f.write_fmt(format_args!("Entering grades: {}-{}", range.start(), range.end()))
                }
            }
            Requirement::CounselorRecommendation => {
                f.write_fmt(format_args!("Recommendation from a counselor"))
            }
            Requirement::SocialSecurityNumber => {
                f.write_fmt(format_args!("Social security number"))
            }
            Requirement::MinWeightedGPA(num) => {
                f.write_fmt(format_args!("Minimum weighted GPA: {num}"))
            }
            Requirement::MinUnweightedGPA(num) => {
                f.write_fmt(format_args!("Minimum unweighted GPA: {num}"))
            }
            Requirement::TestScores(str) => {
                f.write_fmt(format_args!("Test scores: {str}"))
            }
            Requirement::UnofficialTranscript => {
                f.write_fmt(format_args!("Unofficial transcript"))
            }
            Requirement::OfficialTranscript => {
                f.write_fmt(format_args!("Official transcript"))
            }
            Requirement::RecommendationForm(str) => {
                f.write_fmt(format_args!("Recommedation form from {}", str))
            }
            Requirement::LetterOfRec(str) => {
                f.write_fmt(format_args!("Letter of recommendation from {}", str))
            }
            Requirement::Custom(str) => {
                f.write_fmt(format_args!("Other requirement: {}", str))
            }
        }
        
    }
}