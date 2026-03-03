#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(margin: 1.2cm)
#set text(size: spec.profile.customisations.layout.font_size * 1pt)

= Math Answer Key
*Student:* #spec.profile.name \
Date: #spec.profile.customisations.header.date

#let render_answer(item, number) = {
  let lines = item.question.split("\n")
  if lines.len() == 1 {
    [#text(str(number) + ". " + lines.at(0) + "  Answer: " + item.answer)]
  } else {
    #grid(columns: (auto, 1fr, auto), gutter: 0.2cm)[
      [#text(str(number) + ".")]
      [#stack(spacing: 0.05cm, lines.map(l => [l]))]
      [#text("Answer: " + item.answer)]
    ]
  }
}

#columns(2, gutter: 0.8cm)[
  #for section in spec.sections {
    if section.type == "item" {
      [
        #render_answer(section.item, section.number)
        #linebreak()
      ]
    } else {
      [
        #text(section.content)
        #linebreak()
      ]
    }
  }
]
