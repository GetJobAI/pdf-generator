use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use utoipa::ToSchema;

/// Visual style variant for the resume template.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, IntoStaticStr, ToSchema,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Style {
    Minimal,
    Technical,
    #[default]
    Professional,
}

/// Complete resume data sent to the PDF generator.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = resume_example)]
pub struct ResumeData {
    /// Template style: `"professional"` (default), `"minimal"`, or `"technical"`.
    #[schema(example = "professional")]
    pub style: Option<Style>,

    /// Override section heading labels. Useful for non-English resumes.
    pub headings: Option<SectionHeadings>,

    pub contact: Contact,

    /// Professional summary. Supports inline markup: `**bold**`, `_italic_`, `` `code` ``.
    #[schema(
        example = "Backend engineer with 5 years of experience building distributed systems in *Rust* and *Python*."
    )]
    pub summary: Option<String>,

    #[serde(default)]
    pub experience: Vec<ExperienceEntry>,

    #[serde(default)]
    pub education: Vec<EducationEntry>,

    #[serde(default)]
    pub skills: Vec<SkillGroup>,

    #[serde(default)]
    pub certifications: Vec<Certification>,

    #[serde(default)]
    pub projects: Vec<Project>,

    #[serde(default)]
    pub languages: Vec<Language>,
}

/// Custom labels for each resume section.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SectionHeadings {
    #[schema(example = "Summary")]
    pub summary: Option<String>,
    #[schema(example = "Experience")]
    pub experience: Option<String>,
    #[schema(example = "Education")]
    pub education: Option<String>,
    #[schema(example = "Skills")]
    pub skills: Option<String>,
    #[schema(example = "Certifications")]
    pub certifications: Option<String>,
    #[schema(example = "Projects")]
    pub projects: Option<String>,
    #[schema(example = "Languages")]
    pub languages: Option<String>,
}

/// Contact information displayed at the top of the resume.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Contact {
    #[schema(example = "Jane Doe")]
    pub name: String,
    #[schema(example = "jane@example.com")]
    pub email: Option<String>,
    #[schema(example = "+49 170 123 4567")]
    pub phone: Option<String>,
    #[schema(example = "Berlin, Germany")]
    pub location: Option<String>,
    /// Domain only — `https://` is prepended by the template.
    #[schema(example = "linkedin.com/in/janedoe")]
    pub linkedin: Option<String>,
    /// Domain only — `https://` is prepended by the template.
    #[schema(example = "github.com/janedoe")]
    pub github: Option<String>,
}

/// A single work experience entry.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExperienceEntry {
    /// Omit for career-gap entries.
    #[schema(example = "Acme GmbH")]
    pub company: Option<String>,
    /// Omit for career-gap entries.
    #[schema(example = "Senior Software Engineer")]
    pub title: Option<String>,
    #[schema(example = "03.2022 – present")]
    pub dates: String,
    pub location: Option<String>,
    /// Each bullet supports inline markup.
    #[serde(default)]
    pub bullets: Vec<String>,
    /// Set `true` to suppress this entry without removing it from the data.
    #[serde(default)]
    pub hide: bool,
}

/// A single education entry.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EducationEntry {
    #[schema(example = "Lviv Polytechnic National University")]
    pub institution: String,
    #[schema(example = "M.Sc. Computer Science")]
    pub degree: String,
    #[schema(example = "09.2016 – 06.2021")]
    pub dates: String,
    pub location: Option<String>,
    /// Free-form grade string, e.g. `"5.0 / 5.0"`, `"1.3 (DE)"`, `"94 / 100"`.
    pub grade: Option<String>,
    /// Set `true` to suppress this entry without removing it from the data.
    #[serde(default)]
    pub hide: bool,
}

/// A skill category with its items.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SkillGroup {
    #[schema(example = "Languages")]
    pub category: String,
    pub items: Vec<String>,
}

/// A single certification entry.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Certification {
    #[schema(example = "AWS Solutions Architect – Associate")]
    pub name: String,
    #[schema(example = "Amazon Web Services")]
    pub issuer: String,
    #[schema(example = "11.2023")]
    pub date: String,
}

/// A project entry.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Project {
    #[schema(example = "typst-resume")]
    pub name: String,
    /// Supports inline markup.
    #[schema(example = "Open-source ATS-safe resume template for *Typst*. 200+ GitHub stars.")]
    pub description: String,
    /// Domain only — `https://` is prepended by the template.
    #[schema(example = "github.com/janedoe/typst-resume")]
    pub url: Option<String>,
}

/// A language proficiency entry.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Language {
    #[schema(example = "English")]
    pub name: String,
    /// Proficiency level. CEFR scale preferred (A1–C2 / Native), but free-form is accepted.
    #[schema(example = "C1")]
    pub level: String,
}

pub fn resume_example() -> serde_json::Value {
    serde_json::to_value(ResumeData {
        style: Some(Style::Professional),
        headings: Some(SectionHeadings {
            summary: Some("Summary".into()),
            experience: Some("Experience".into()),
            education: Some("Education".into()),
            skills: Some("Skills".into()),
            certifications: Some("Certifications".into()),
            projects: Some("Projects".into()),
            languages: Some("Languages".into())
        }),
        contact: Contact {
            name: "Jane Doe".into(),
            email: Some("jane@example.com".into()),
            phone: Some("+49 170 123 4567".into()),
            location: Some("Berlin, Germany".into()),
            linkedin: Some("linkedin.com/in/janedoe".into()),
            github: Some("github.com/janedoe".into()),
        },
        summary: Some(
            "Backend engineer with 5 years of experience building distributed systems in *Rust* and *Python*. \
             Focused on high-throughput data pipelines and cloud-native architecture."
                .into(),
        ),
        experience: vec![
            ExperienceEntry {
                company: Some("Acme GmbH".into()),
                title: Some("Senior Software Engineer".into()),
                dates: "03.2022 – present".into(),
                location: Some("Berlin, Germany".into()),
                bullets: vec![
                    "Built distributed ingestion pipeline using *Rust* and *Apache Kafka*, reducing p99 latency by 40%.".into(),
                    "Led migration of 3 legacy services to microservices, enabling independent deployments across teams.".into(),
                    "Mentored 2 junior engineers through weekly code review and pair programming sessions.".into(),
                ],
                hide: false,
            },
            ExperienceEntry {
                company: Some("Startup OÜ".into()),
                title: Some("Software Engineer".into()),
                dates: "06.2019 – 02.2022".into(),
                location: Some("Tallinn, Estonia".into()),
                bullets: vec![
                    "Implemented OAuth 2.0 login flow using *FastAPI* and *PostgreSQL*, serving 50k monthly active users.".into(),
                    "Reduced CI pipeline from 18 min to 6 min by parallelising test suites in *GitHub Actions*.".into(),
                ],
                hide: false,
            },
        ],
        education: vec![EducationEntry {
            institution: "Lviv Polytechnic National University".into(),
            degree: "M.Sc. Computer Science".into(),
            dates: "09.2016 – 06.2021".into(),
            location: Some("Lviv, Ukraine".into()),
            grade: Some("5.0 / 5.0".into()),
            hide: false,
        }],
        skills: vec![
            SkillGroup {
                category: "Languages".into(),
                items: vec!["Rust".into(), "Python".into(), "TypeScript".into(), "SQL".into()],
            },
            SkillGroup {
                category: "Infrastructure".into(),
                items: vec!["Kafka".into(), "PostgreSQL".into(), "Docker".into(), "Kubernetes".into()],
            },
            SkillGroup {
                category: "Concepts".into(),
                items: vec!["Microservices".into(), "Event-Driven Architecture".into(), "REST".into(), "CI/CD".into()],
            },
        ],
        certifications: vec![Certification {
            name: "AWS Solutions Architect – Associate".into(),
            issuer: "Amazon Web Services".into(),
            date: "11.2023".into(),
        }],
        projects: vec![Project {
            name: "typst-resume".into(),
            description: "Open-source ATS-safe resume template for *Typst*. 200+ GitHub stars.".into(),
            url: Some("github.com/janedoe/typst-resume".into()),
        }],
        languages: vec![
            Language { name: "Ukrainian".into(), level: "C2".into() },
            Language { name: "English".into(), level: "C1".into() },
            Language { name: "German".into(), level: "A2".into() },
        ],
    })
    .expect("ResumeData example is always serialisable")
}
