#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(width: 29.7cm, height: 21cm, margin: 0.45cm)
#set text(size: spec.profile.customisations.layout.font_size * 0.9pt)
#set par(leading: 0.12em, spacing: 0em)

#set par(leading: 0.35em, spacing: 0em)
#grid(columns: (1fr, auto))[
  #text(size: spec.profile.customisations.layout.font_size * 1.05pt, weight: "bold")[Math Worksheet]
][
  #align(right)[
    #text(size: spec.profile.customisations.layout.font_size * 0.85pt)[Date: #spec.profile.customisations.header.date]
  ]
]
#v(0.4cm)
#set par(leading: 0.12em, spacing: 0em)

#let total = spec.sections.len()
#let rows = 33
#let cols = 3
#let body_height = 19.3cm

#context[
  #let item_height = measure(text("1. 1 + 1 = __")).height
  #let raw_gap = (body_height - rows * item_height) / (rows - 1)
  #let gap = calc.max(raw_gap, 2pt)

  #let render_column(start) = block(height: body_height)[
    #for i in range(0, rows) {
      let idx = start + i
      if idx < total {
        let section = spec.sections.at(idx)
        if section.type == "item" {
          let q = section.item.question.replace("\n", " ")
          text(str(section.number) + ". " + q)
        } else {
          text(section.content)
        }
      } else {
        []
      }
      if i < rows - 1 {
        v(gap)
      }
    }
  ]

  #columns(cols, gutter: 0.4cm)[
    #render_column(0)
    #render_column(rows)
    #render_column(rows * 2)
  ]
]
