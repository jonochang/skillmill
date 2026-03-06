#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(margin: (x: 1.4cm, y: 1.2cm))
#set text(
  font: ("Arial", "Helvetica Neue"),
  size: spec.profile.customisations.layout.font_size * 0.96pt,
)
#set par(leading: 0.4em, spacing: 0em)

#let ccs_blue = rgb("#2d86c9")
#let ccs_red = rgb("#e85a4f")
#let ccs_ink = rgb("#222222")
#let ccs_rule = rgb("#7d7d7d")

#let total_sections = spec.sections.len()
#let rows_per_page = 15
#let page_count = calc.ceil(total_sections / rows_per_page)
#let body_height = 22.2cm
#let row_height = body_height / rows_per_page
#let rule_width = 0.05cm
#let answer_column_width = 3.0cm

#let render_header() = [
  #grid(columns: (1fr, 2.9cm), align: (left, bottom), column-gutter: 0.5cm)[
    #stack(spacing: 0.05cm)[
      #rect(width: 1.15cm, height: 0.18cm, radius: 0.07cm, fill: ccs_blue)
      #place(dx: 0.18cm, dy: -0.03cm, rect(width: 0.42cm, height: 0.18cm, radius: 0.07cm, fill: ccs_red))
      #text(fill: ccs_ink, weight: "bold", size: spec.profile.customisations.layout.font_size * 1.08pt)[Worksheet Answer Key]
    ]
  ][
    #text(fill: ccs_ink, weight: "bold")[Date:]
    #v(0.12cm)
    #box(width: 100%)[#spec.profile.customisations.header.date]
  ]
  #v(0.14cm)
  #line(length: 100%, stroke: (paint: ccs_ink, thickness: 0.9pt))
  #v(0.22cm)
]

#let render_vertical_rule() = box(width: rule_width, height: row_height)[
  #align(center + horizon)[#line(length: row_height, angle: 90deg, stroke: (paint: ccs_rule, thickness: 0.7pt))]
]

#let render_answer_box(number, answer_text) = box(width: answer_column_width, height: row_height)[
  #grid(columns: (0.65cm, 1fr), align: (left, center), column-gutter: 0.18cm)[
    #text(fill: ccs_rule, size: spec.profile.customisations.layout.font_size * 0.82pt)[#number.]
  ][
    #align(left + horizon)[#text(fill: ccs_red, weight: "medium")[#answer_text]]
  ]
]

#let render_item_row(section) = grid(
  columns: (1fr, rule_width, answer_column_width),
  align: (left, center),
  column-gutter: 0.28cm,
)[
  #box(height: row_height, width: 100%)[#align(left + horizon)[#text(str(section.number) + ") " + section.item.question)]]
][
  #render_vertical_rule()
][
  #render_answer_box(section.number, section.item.answer)
]

#let render_custom_row(section) = grid(
  columns: (1fr, rule_width, answer_column_width),
  align: (left, center),
  column-gutter: 0.28cm,
)[
  #box(height: row_height, width: 100%)[#align(left + horizon)[#text(style: "italic")[#section.content]]]
][
  #render_vertical_rule()
][
  #box(width: answer_column_width, height: row_height)[]
]

#let render_slot(global_idx) = {
  if global_idx < total_sections {
    let section = spec.sections.at(global_idx)
    if section.type == "item" {
      render_item_row(section)
    } else {
      render_custom_row(section)
    }
  } else {
    box(height: row_height, width: 100%)[]
  }
}

#for page in range(0, page_count) {
  let page_start = page * rows_per_page
  render_header()
  box(height: body_height, width: 100%)[
    #for row in range(0, rows_per_page) {
      render_slot(page_start + row)
    }
  ]

  if page < page_count - 1 {
    pagebreak()
  }
}
