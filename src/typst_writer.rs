//! Serialises a [`ResumeData`] value into a complete Typst source string ready
//! for compilation. The output imports `template.typ` and calls `#resume(...)`.

use crate::resume::{
    Certification, Contact, EducationEntry, ExperienceEntry, Language, Project, ResumeData,
    SectionHeadings, SkillGroup,
};
use crate::str_to_content::str_to_content;

/// Returns the full Typst source for the given resume data.
pub fn render(data: &ResumeData) -> String {
    let mut parts: Vec<String> = Vec::new();

    let style = data.style.unwrap_or_default().into();
    parts.push(format!("style: {}", typst_str(style)));

    if let Some(headings) = &data.headings {
        parts.push(format!("headings: {}", render_headings(headings)));
    }

    parts.push(format!("contact: {}", render_contact(&data.contact)));

    if let Some(summary) = &data.summary {
        parts.push(format!("summary: {}", str_to_content(summary)));
    }

    if !data.experience.is_empty() {
        let rendered: Vec<String> = data.experience.iter().map(render_experience).collect();
        parts.push(format!("experience: {}", typst_array(&rendered)));
    }

    if !data.education.is_empty() {
        let rendered: Vec<String> = data.education.iter().map(render_education).collect();
        parts.push(format!("education: {}", typst_array(&rendered)));
    }

    if !data.skills.is_empty() {
        let rendered: Vec<String> = data.skills.iter().map(render_skill_group).collect();
        parts.push(format!("skills: {}", typst_array(&rendered)));
    }

    if !data.certifications.is_empty() {
        let rendered: Vec<String> = data
            .certifications
            .iter()
            .map(render_certification)
            .collect();
        parts.push(format!("certifications: {}", typst_array(&rendered)));
    }

    if !data.projects.is_empty() {
        let rendered: Vec<String> = data.projects.iter().map(render_project).collect();
        parts.push(format!("projects: {}", typst_array(&rendered)));
    }

    if !data.languages.is_empty() {
        let rendered: Vec<String> = data.languages.iter().map(render_language).collect();
        parts.push(format!("languages: {}", typst_array(&rendered)));
    }

    format!(
        "#import \"template.typ\": resume\n#resume(({}))",
        parts.join(", "),
    )
}

/// Wraps a string in a Typst string literal, escaping `\` and `"`.
fn typst_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Produces a Typst array literal from pre-rendered element strings.
///
/// Single-element arrays require a trailing comma so Typst does not treat
/// `(expr)` as a parenthesised expression rather than an array.
fn typst_array(items: &[String]) -> String {
    match items.len() {
        0 => "()".to_owned(),
        1 => format!("({},)", items[0]),
        _ => format!("({})", items.join(", ")),
    }
}

fn render_headings(h: &SectionHeadings) -> String {
    let mut fields: Vec<String> = Vec::new();
    if let Some(s) = &h.summary {
        fields.push(format!("summary: {}", typst_str(s)));
    }
    if let Some(s) = &h.experience {
        fields.push(format!("experience: {}", typst_str(s)));
    }
    if let Some(s) = &h.education {
        fields.push(format!("education: {}", typst_str(s)));
    }
    if let Some(s) = &h.skills {
        fields.push(format!("skills: {}", typst_str(s)));
    }
    if let Some(s) = &h.certifications {
        fields.push(format!("certifications: {}", typst_str(s)));
    }
    if let Some(s) = &h.projects {
        fields.push(format!("projects: {}", typst_str(s)));
    }
    if let Some(s) = &h.languages {
        fields.push(format!("languages: {}", typst_str(s)));
    }
    if fields.is_empty() {
        "(:)".to_owned()
    } else {
        format!("({})", fields.join(", "))
    }
}

fn render_contact(c: &Contact) -> String {
    let mut fields: Vec<String> = Vec::new();
    fields.push(format!("name: {}", typst_str(&c.name)));
    if let Some(v) = &c.email {
        fields.push(format!("email: {}", typst_str(v)));
    }
    if let Some(v) = &c.phone {
        fields.push(format!("phone: {}", typst_str(v)));
    }
    if let Some(v) = &c.location {
        fields.push(format!("location: {}", typst_str(v)));
    }
    if let Some(v) = &c.linkedin {
        fields.push(format!("linkedin: {}", typst_str(v)));
    }
    if let Some(v) = &c.github {
        fields.push(format!("github: {}", typst_str(v)));
    }
    format!("({})", fields.join(", "))
}

fn render_experience(e: &ExperienceEntry) -> String {
    let mut fields: Vec<String> = Vec::new();
    if let Some(v) = &e.company {
        fields.push(format!("company: {}", typst_str(v)));
    }
    if let Some(v) = &e.title {
        fields.push(format!("title: {}", typst_str(v)));
    }
    fields.push(format!("dates: {}", typst_str(&e.dates)));
    if let Some(v) = &e.location {
        fields.push(format!("location: {}", typst_str(v)));
    }
    let bullets: Vec<String> = e.bullets.iter().map(|b| str_to_content(b)).collect();
    fields.push(format!("bullets: {}", typst_array(&bullets)));
    if e.hide {
        fields.push("hide: true".to_owned());
    }
    format!("({})", fields.join(", "))
}

fn render_education(e: &EducationEntry) -> String {
    let mut fields: Vec<String> = Vec::new();
    fields.push(format!("institution: {}", typst_str(&e.institution)));
    fields.push(format!("degree: {}", typst_str(&e.degree)));
    fields.push(format!("dates: {}", typst_str(&e.dates)));
    if let Some(v) = &e.location {
        fields.push(format!("location: {}", typst_str(v)));
    }
    if let Some(v) = &e.grade {
        fields.push(format!("grade: {}", typst_str(v)));
    }
    if e.hide {
        fields.push("hide: true".to_owned());
    }
    format!("({})", fields.join(", "))
}

fn render_skill_group(s: &SkillGroup) -> String {
    let items: Vec<String> = s.items.iter().map(|i| typst_str(i)).collect();
    format!(
        "(category: {}, items: {})",
        typst_str(&s.category),
        typst_array(&items),
    )
}

fn render_certification(c: &Certification) -> String {
    format!(
        "(name: {}, issuer: {}, date: {})",
        typst_str(&c.name),
        typst_str(&c.issuer),
        typst_str(&c.date),
    )
}

fn render_project(p: &Project) -> String {
    let mut fields: Vec<String> = Vec::new();
    fields.push(format!("name: {}", typst_str(&p.name)));
    fields.push(format!("description: {}", str_to_content(&p.description)));
    if let Some(v) = &p.url {
        fields.push(format!("url: {}", typst_str(v)));
    }
    format!("({})", fields.join(", "))
}

fn render_language(l: &Language) -> String {
    format!(
        "(name: {}, level: {})",
        typst_str(&l.name),
        typst_str(&l.level),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::resume::Style;

    fn minimal() -> ResumeData {
        ResumeData {
            style: None,
            headings: None,
            contact: Contact {
                name: "Jane Doe".to_owned(),
                email: None,
                phone: None,
                location: None,
                linkedin: None,
                github: None,
            },
            summary: None,
            experience: vec![],
            education: vec![],
            skills: vec![],
            certifications: vec![],
            projects: vec![],
            languages: vec![],
        }
    }

    #[test]
    fn has_import_and_contact_name() {
        let src = render(&minimal());
        assert!(src.contains("#import \"template.typ\": resume"));
        assert!(src.contains("name: \"Jane Doe\""));
    }

    #[test]
    fn style_defaults_to_professional() {
        assert!(render(&minimal()).contains("style: \"professional\""));
    }

    #[test]
    fn explicit_style_written() {
        let mut data = minimal();
        data.style = Some(Style::Technical);
        assert!(render(&data).contains("style: \"technical\""));
    }

    #[test]
    fn summary_markup_preserved() {
        let mut data = minimal();
        data.summary = Some("Expert in *Rust*.".to_owned());
        assert!(render(&data).contains("#strong[Rust]"));
    }

    #[test]
    fn single_experience_has_array_trailing_comma() {
        let mut data = minimal();
        data.experience = vec![ExperienceEntry {
            company: Some("Acme".to_owned()),
            title: None,
            dates: "2020 – present".to_owned(),
            location: None,
            bullets: vec![],
            hide: false,
        }];
        let src = render(&data);
        // Single-element array: (...,)
        assert!(src.contains("experience: (("));
        assert!(src.contains(",)"));
    }

    #[test]
    fn hidden_entry_emits_hide_true() {
        let mut data = minimal();
        data.experience = vec![ExperienceEntry {
            company: Some("Acme".to_owned()),
            title: None,
            dates: "2020 – 2021".to_owned(),
            location: None,
            bullets: vec![],
            hide: true,
        }];
        assert!(render(&data).contains("hide: true"));
    }

    #[test]
    fn omitted_sections_absent() {
        let src = render(&minimal());
        assert!(!src.contains("experience:"));
        assert!(!src.contains("education:"));
    }

    #[test]
    fn strings_with_quotes_escaped() {
        let mut data = minimal();
        data.contact.name = r#"O'Brien "Bob""#.to_owned();
        assert!(render(&data).contains(r#"name: "O'Brien \"Bob\"""#));
    }
}
