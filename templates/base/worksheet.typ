#let data = json(bytes(sys.inputs.data))
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
#let problem_count = spec.items.len()
#let rows_per_page = 15
#let page_count = calc.ceil(total_sections / rows_per_page)
#let body_height = 22.2cm
#let row_height = body_height / rows_per_page
#let rule_width = 0.05cm
#let answer_column_width = 3.0cm

#let blank_line(width) = box(width: width, height: 1.1em)[
  #align(bottom)[#line(length: 100%, stroke: 0.7pt)]
]

#let render_header() = [
  #grid(columns: (1fr, 3.8cm, 2.9cm), align: (left, bottom), column-gutter: 0.5cm)[
    #stack(spacing: 0.05cm)[
      #rect(width: 1.15cm, height: 0.18cm, radius: 0.07cm, fill: ccs_blue)
      #place(dx: 0.18cm, dy: -0.03cm, rect(width: 0.42cm, height: 0.18cm, radius: 0.07cm, fill: ccs_red))
      #text(fill: ccs_ink, weight: "bold", size: spec.profile.customisations.layout.font_size * 1.08pt)[Worksheet]
    ]
  ][
    #text(fill: ccs_ink, weight: "bold")[Name:]
    #v(0.12cm)
    #if spec.profile.name == "" { blank_line(100%) } else { box(width: 100%)[#spec.profile.name] }
  ][
    #text(fill: ccs_ink, weight: "bold")[Date:]
    #v(0.12cm)
    #if spec.profile.customisations.header.date == "" {
      blank_line(100%)
    } else {
      box(width: 100%)[#spec.profile.customisations.header.date]
    }
  ]
  #v(0.14cm)
  #line(length: 100%, stroke: (paint: ccs_ink, thickness: 0.9pt))
  #v(0.22cm)
]

#let render_vertical_rule() = box(width: rule_width, height: row_height)[
  #align(center + horizon)[#line(length: row_height, angle: 90deg, stroke: (paint: ccs_rule, thickness: 0.7pt))]
]

#let render_answer_box(number) = box(width: answer_column_width, height: row_height)[
  #grid(columns: (0.65cm, 1fr), align: (left, center), column-gutter: 0.18cm)[
    #text(fill: ccs_rule, size: spec.profile.customisations.layout.font_size * 0.82pt)[#number.]
  ][
    #blank_line(100%)
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
  #render_answer_box(section.number)
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

#let render_footer() = [
  #if problem_count > 0 {
    let missed_cells = range(0, problem_count + 1).map(missed => [
      #text(size: spec.profile.customisations.layout.font_size * 0.72pt)[#missed]
    ])
    let score_cells = range(0, problem_count + 1).map(missed => {
      let pct = calc.round((problem_count - missed) * 100 / problem_count)
      [#text(size: spec.profile.customisations.layout.font_size * 0.72pt)[#pct%]]
    })
    table(
      columns: (1fr,) * (problem_count + 2),
      align: center,
      inset: 0.12cm,
      stroke: 0.45pt,
      [#text(fill: ccs_blue, weight: "semibold", size: spec.profile.customisations.layout.font_size * 0.74pt)[Missed]],
      ..missed_cells,
      [#text(fill: ccs_red, weight: "semibold", size: spec.profile.customisations.layout.font_size * 0.74pt)[Score]],
      ..score_cells,
    )
  }
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
  v(0.25cm)
  render_footer()

  if page < page_count - 1 {
    pagebreak()
  }
}
