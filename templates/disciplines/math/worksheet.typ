#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(margin: 1.2cm)
#set text(size: spec.profile.customisations.layout.font_size * 1pt)

= Math Worksheet
#let school = spec.profile.customisations.header.school
#let class = spec.profile.customisations.header.class
*Student:* #spec.profile.name \
School: #if school == none { "" } else { school } \
Class: #if class == none { "" } else { class } \
Date: #spec.profile.customisations.header.date

#let working_space(size) = {
  let height = if size == "small" { 0.7cm } else if size == "large" { 1.4cm } else { 1.0cm }
  rect(width: 100%, height: height, stroke: 0.4pt + gray)
}

#let render_question(item, number) = {
  let lines = item.question.split("\n")
  if lines.len() == 1 {
    [#text(str(number) + ". " + lines.at(0))]
  } else {
    #grid(columns: (auto, 1fr), gutter: 0.2cm)[
      [#text(str(number) + ".")]
      [#stack(spacing: 0.05cm, lines.map(l => [l]))]
    ]
  }
}

#columns(2, gutter: 0.8cm)[
  #for section in spec.sections {
    if section.type == "item" {
      [
        #render_question(section.item, section.number)
        #linebreak()
        #working_space(spec.profile.customisations.layout.working_space)
        #linebreak()
        #linebreak()
      ]
    } else {
      [
        #text(section.content)
        #linebreak()
        #linebreak()
      ]
    }
  }
]
