/// GetJobAI resume template: three styles, single template.
/// See README.md for full usage, data schema, and credits.

/// Theme definitions
#let themes = (
  minimal: (
    font: ("New Computer Modern", "Libertinus Serif"),
    accent: black,
    muted: luma(100),
    rule-stroke: 0.4pt + luma(180),
    name-size: 20pt,
    section-size: 10.5pt,
    body-size: 10pt,
    entry-gap: 6pt,
    section-above: 10pt,
    section-below: 4pt,
    margin-x: 1.8cm,
    separator: " · ",
  ),
  technical: (
    // mono body, wider margins eat into line length, so pull them in a bit
    font: ("JetBrains Mono", "Libertinus Serif"),
    accent: rgb("#005f87"),
    muted: rgb("#555555"),
    rule-stroke: 1pt + rgb("#005f87"),
    name-size: 13pt,
    section-size: 9.5pt,
    body-size: 9pt,
    entry-gap: 4pt,
    section-above: 7pt,
    section-below: 3pt,
    margin-x: 1.2cm,
    separator: " · ", // single-space as monospace makes double-space very wide
  ),
  professional: (
    font: ("Libertinus Serif",),
    accent: rgb("#1a3a5c"),
    muted: luma(80),
    rule-stroke: 0.6pt + rgb("#1a3a5c"),
    name-size: 24pt,
    section-size: 11pt,
    body-size: 10.5pt,
    entry-gap: 7pt,
    section-above: 12pt,
    section-below: 3pt,
    margin-x: 1.8cm,
    separator: "  ·  ",
  ),
)

/// Default section headings
#let default-headings = (
  summary: "Summary",
  experience: "Experience",
  education: "Education",
  skills: "Skills",
  certifications: "Certifications",
  projects: "Projects",
  languages: "Languages",
)

// Diff view markers, no-ops when not in a diff preview.
#let diff-added(it) = text(fill: rgb("#15803d"), underline(it))
#let diff-deleted(it) = text(fill: rgb("#b91c1c"), strike(it))

/// Section heading with a rule underneath. Always use standard English names
/// ("Experience", "Education", etc.) so ATS classifies sections correctly.
#let section-heading(title, theme) = {
  v(theme.section-above)

  text(
    size: theme.section-size,
    weight: "bold",
    fill: theme.accent,
    upper(title),
  )

  v(2pt)

  line(
    length: 100%,
    stroke: theme.rule-stroke,
  )

  v(theme.section-below)
}


/// Contact block: must be at the top of the body, not in a PDF header/footer,
/// so parsers don't skip or misattribute it. All fields except name are optional.
#let contact-block(contacts, theme) = {
  text(
    size: theme.name-size,
    weight: "bold",
    fill: theme.accent,
    contacts.name,
  )

  v(7pt)

  // box() prevents line breaks inside URLs (Typst breaks at '/' in links by default)
  let parts = ()

  if contacts.at("location", default: none) != none {
    parts.push(box(contacts.location))
  }
  if contacts.at("email", default: none) != none {
    parts.push(box(link("mailto:" + contacts.email, contacts.email)))
  }
  if contacts.at("phone", default: none) != none {
    parts.push(box(contacts.phone))
  }
  if contacts.at("linkedin", default: none) != none {
    parts.push(box(link("https://" + contacts.linkedin, contacts.linkedin)))
  }
  if contacts.at("github", default: none) != none {
    parts.push(box(link("https://" + contacts.github, contacts.github)))
  }

  // Iterate to capture other generic contacts like WhatsApp, Signal, Portfolio, etc.
  let known-keys = ("name", "location", "email", "phone", "linkedin", "github")
  for (key, val) in contacts {
    if key not in known-keys and val != none {
      parts.push(box(val))
    }
  }

  text(
    size: theme.body-size - 0.5pt,
    parts.join(theme.separator),
  )

  v(theme.section-above * 0.6)
}

/// Work experience entry.
/// Company and title are optional, omit both for a career-gap entry.
/// Set hide: true to suppress an entry without removing it from the data.
#let work-entry(entry, theme) = {
  if entry.at("hide", default: false) {
    return
  }

  block(below: theme.entry-gap)[
    #if entry.at("company", default: none) != none {
      strong(entry.company)
    }
    #h(1fr)
    #entry.dates
    \
    #if entry.at("title", default: none) != none {
      emph(entry.title)
    }
    #if entry.at("location", default: none) != none {
      h(1fr)
      text(
        size: theme.body-size - 0.5pt,
        fill: theme.muted,
      )[#entry.location]
    }
    #v(3pt)
    #list(..entry.bullets)
  ]
}

/// Education entry.
/// grade is a free-form string: "5.0 / 5.0", "1.3 (DE)", "94 / 100".
/// Set hide: true to suppress without removing from data.
#let education-entry(entry, theme) = {
  if entry.at("hide", default: false) {
    return
  }

  block(below: theme.entry-gap)[
    #strong(entry.institution)
    #h(1fr)
    #entry.dates
    \
    #emph(entry.degree)
    #if entry.at("location", default: none) != none {
      h(1fr)
      text(
        size: theme.body-size - 0.5pt,
        fill: theme.muted,
      )[#entry.location]
    }
    #if entry.at("grade", default: none) != none {
      [\ ]
      text(size: theme.body-size - 0.5pt)[Grade: #entry.grade]
    }
  ]
}

/// Skills section as linear text: "Category: item, item, item, ..."
#let skills-section(groups) = {
  for group in groups {
    block(below: 3pt)[
      #strong(group.category + ": ")#group.items.join(", ")
    ]
  }
}

/// Single certification on one line.
#let certification-entry(cert, theme) = block(below: 3pt)[
  #strong(cert.name)#theme.separator#cert.issuer #h(1fr) #cert.date
]

/// Project entry.
#let project-entry(project, theme) = block(below: theme.entry-gap)[
  #strong(project.name)
  #if project.at("url", default: none) != none {
    text(fill: theme.muted)[ — ]
    link("https://" + project.url)[
      #text(size: theme.body-size - 0.5pt)[#project.url]
    ]
  }
  \
  #project.description
]

/// Languages as a single readable line.
#let languages-line(langs, theme) = (
  langs.map(l => l.name + " — " + l.level).join(theme.separator)
)


#let resume(data) = {
  let style-name = sys.inputs.at(
    "style", // cmd arg precedence: --input style=X
    default: data.at("style", default: "professional"),
  )

  let theme = themes.at(style-name, default: themes.professional)
  let headings = data.at("headings", default: (:))
  let headings = default-headings + if type(headings) == dictionary { headings } else { (:) }

  set page(
    paper: "a4",
    margin: (
      x: theme.margin-x,
      y: 2cm,
    ),
  )

  set text(
    font: theme.font,
    size: theme.body-size,
    lang: "en",
    hyphenate: false,
  )

  set par(
    justify: false, // justified text can confuse some parsers with uneven spacing
    leading: 0.75em,
    spacing: 0em,
  )

  set list(
    indent: 0pt,
    marker: [•],
    body-indent: 0.6em,
  )
  show list: set block(
    above: 4pt,
    below: 0pt,
  )
  show list: set par(
    leading: 0.6em,
  )

  // Document body: sections are omitted when their data arrays are empty or absent.

  contact-block(data.contact, theme)

  if data.at("summary", default: none) != none {
    section-heading(headings.summary, theme)
    data.summary
  }

  let experience = data.at("experience", default: ())
  if experience.len() > 0 {
    section-heading(headings.experience, theme)
    for exp in experience {
      work-entry(exp, theme)
    }
  }

  let education = data.at("education", default: ())
  if education.len() > 0 {
    section-heading(headings.education, theme)
    for edu in education {
      education-entry(edu, theme)
    }
  }

  let skills = data.at("skills", default: ())
  if skills.len() > 0 {
    section-heading(headings.skills, theme)
    skills-section(skills)
  }

  let certifications = data.at("certifications", default: ())
  if certifications.len() > 0 {
    section-heading(headings.certifications, theme)
    for cert in certifications {
      certification-entry(cert, theme)
    }
  }

  let projects = data.at("projects", default: ())
  if projects.len() > 0 {
    section-heading(headings.projects, theme)
    for project in projects {
      project-entry(project, theme)
    }
  }

  let languages = data.at("languages", default: ())
  if languages.len() > 0 {
    section-heading(headings.languages, theme)
    languages-line(languages, theme)
  }
}
