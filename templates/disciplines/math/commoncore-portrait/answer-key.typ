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
#let ccs_fill = rgb("#eef7ff")

#let total_sections = spec.sections.len()
#let uses_visual_layout = spec.policy.target_node == "p1-geometry-2d-sides" or spec.policy.target_node == "p2-fractions-identify-shaded" or spec.policy.target_node == "p2-geometry-2d-vertices" or spec.policy.target_node == "p3-geometry-3d-faces"
#let rows_per_page = if uses_visual_layout { 8 } else { 15 }
#let page_count = calc.ceil(total_sections / rows_per_page)
#let body_height = 22.2cm
#let row_height = body_height / rows_per_page
#let number_width = 1.1cm
#let rule_width = 0.05cm
#let answer_column_width = 3.6cm
#let stack_width = 3.9cm
#let visual_width = 2.5cm

#let header_date = spec.profile.customisations.header.date

#let worksheet_title() = {
  let node = spec.policy.target_node
  if node == "p1-numbers-count-to-100" {
    "Count to 100"
  } else if node == "p1-add-sub-within-10" {
    "Add & Subtract Within 10"
  } else if node == "p1-add-sub-within-20" {
    "Add & Subtract Within 20"
  } else if node == "p1-geometry-2d-sides" {
    "2D Shapes: Number of Sides"
  } else if node == "p2-add-sub-within-100" {
    "Add & Subtract Within 100"
  } else if node == "p2-multiply-2-3-4-5-10" {
    "Multiply Facts"
  } else if node == "p2-divide-2-3-4-5-10" {
    "Divide Facts"
  } else if node == "p2-fractions-identify-shaded" {
    "Fractions: Shaded Parts"
  } else if node == "p2-geometry-2d-vertices" {
    "2D Shapes: Number of Vertices"
  } else if node == "p3-add-sub-within-10000" {
    "Add & Subtract Within 10,000"
  } else if node == "p3-multiply-6-7-8-9" {
    "Multiply Facts"
  } else if node == "p3-divide-6-7-8-9" {
    "Divide Facts"
  } else if node == "p3-geometry-3d-faces" {
    "3D Solids: Number of Faces"
  } else {
    "Math Worksheet"
  }
}

#let answer_key_title() = worksheet_title() + " Answer Key"

#let render_header() = [
  #grid(columns: (1fr, 2.9cm), align: (left, bottom), column-gutter: 0.5cm)[
    #stack(spacing: 0.05cm)[
      #rect(width: 1.15cm, height: 0.18cm, radius: 0.07cm, fill: ccs_blue)
      #place(dx: 0.18cm, dy: -0.03cm, rect(width: 0.42cm, height: 0.18cm, radius: 0.07cm, fill: ccs_red))
      #text(fill: ccs_ink, weight: "bold", size: spec.profile.customisations.layout.font_size * 1.08pt)[#answer_key_title()]
    ]
  ][
    #text(fill: ccs_ink, weight: "bold")[Date:]
    #v(0.12cm)
    #box(width: 100%)[#header_date]
  ]
  #v(0.14cm)
  #text(fill: ccs_red, weight: "semibold")[Answer Key]
  #v(0.12cm)
  #line(length: 100%, stroke: (paint: ccs_ink, thickness: 0.9pt))
  #v(0.22cm)
]

#let render_vertical_rule() = box(width: rule_width, height: row_height)[
  #align(center + horizon)[#line(length: row_height, angle: 90deg, stroke: (paint: ccs_rule, thickness: 0.7pt))]
]

#let render_answer_box(number, answer_text) = box(width: answer_column_width, height: row_height)[
  #grid(
    columns: (0.65cm, 1fr),
    align: (left, center),
    column-gutter: 0.18cm,
  )[
    #text(fill: ccs_rule, size: spec.profile.customisations.layout.font_size * 0.82pt)[#number.]
  ][
    #align(left + horizon)[#text(fill: ccs_red, weight: "medium")[#answer_text]]
  ]
]

#let render_shape_2d(shape) = box(width: visual_width, height: row_height - 0.25cm)[
  #align(center + horizon)[
    #if shape == "triangle" {
      polygon(stroke: (paint: ccs_ink, thickness: 0.9pt), fill: ccs_fill, (1.2cm, 0.1cm), (2.2cm, 1.7cm), (0.2cm, 1.7cm))
    } else if shape == "square" {
      rect(width: 1.7cm, height: 1.7cm, stroke: (paint: ccs_ink, thickness: 0.9pt), fill: ccs_fill)
    } else if shape == "rectangle" {
      rect(width: 2.0cm, height: 1.45cm, stroke: (paint: ccs_ink, thickness: 0.9pt), fill: ccs_fill)
    } else if shape == "pentagon" {
      polygon(stroke: (paint: ccs_ink, thickness: 0.9pt), fill: ccs_fill, (1.2cm, 0.05cm), (2.15cm, 0.75cm), (1.8cm, 1.8cm), (0.6cm, 1.8cm), (0.25cm, 0.75cm))
    } else {
      polygon(stroke: (paint: ccs_ink, thickness: 0.9pt), fill: ccs_fill, (0.55cm, 0.1cm), (1.75cm, 0.1cm), (2.35cm, 0.95cm), (1.75cm, 1.8cm), (0.55cm, 1.8cm), (-0.05cm, 0.95cm))
    }
  ]
]

#let render_solid_3d(solid) = box(width: visual_width, height: row_height - 0.25cm)[
  #align(center + horizon)[
    #if solid == "cube" or solid == "cuboid" {
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: ccs_fill, (0.35cm, 0.55cm), (1.55cm, 0.55cm), (1.55cm, 1.75cm), (0.35cm, 1.75cm))
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: rgb("#ffffff"), (0.95cm, 0.15cm), (2.15cm, 0.15cm), (2.15cm, 1.35cm), (0.95cm, 1.35cm))
      line(start: (0.35cm, 0.55cm), end: (0.95cm, 0.15cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.55cm, 0.55cm), end: (2.15cm, 0.15cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.55cm, 1.75cm), end: (2.15cm, 1.35cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (0.35cm, 1.75cm), end: (0.95cm, 1.35cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
    } else if solid == "triangular-prism" {
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: ccs_fill, (0.35cm, 1.7cm), (1.1cm, 0.25cm), (1.85cm, 1.7cm))
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: rgb("#ffffff"), (0.85cm, 1.35cm), (1.6cm, -0.1cm), (2.35cm, 1.35cm))
      line(start: (0.35cm, 1.7cm), end: (0.85cm, 1.35cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.1cm, 0.25cm), end: (1.6cm, -0.1cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.85cm, 1.7cm), end: (2.35cm, 1.35cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
    } else if solid == "square-pyramid" {
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: ccs_fill, (0.4cm, 1.45cm), (1.45cm, 1.05cm), (2.2cm, 1.6cm), (1.15cm, 1.95cm))
      line(start: (1.25cm, 0.15cm), end: (0.4cm, 1.45cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.25cm, 0.15cm), end: (1.45cm, 1.05cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.25cm, 0.15cm), end: (2.2cm, 1.6cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (1.25cm, 0.15cm), end: (1.15cm, 1.95cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
    } else {
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: ccs_fill, (1.25cm, 0.15cm), (2.2cm, 1.75cm), (0.3cm, 1.75cm))
      polygon(stroke: (paint: ccs_ink, thickness: 0.8pt), fill: rgb("#ffffff"), (1.25cm, 0.65cm), (1.8cm, 1.55cm), (0.7cm, 1.55cm))
      line(start: (1.25cm, 0.15cm), end: (1.25cm, 0.65cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (2.2cm, 1.75cm), end: (1.8cm, 1.55cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
      line(start: (0.3cm, 1.75cm), end: (0.7cm, 1.55cm), stroke: (paint: ccs_ink, thickness: 0.8pt))
    }
  ]
]

#let fraction_cells(parts, shaded, fill_color, height) = range(0, parts).map(idx => rect(
  width: 100%,
  height: height,
  fill: if idx < shaded { fill_color } else { rgb(255, 255, 255) },
))

#let fraction_row(parts, shaded, fill_color, height) = table(
  columns: (1fr,) * parts,
  inset: 0pt,
  stroke: 0.6pt,
  ..fraction_cells(parts, shaded, fill_color, height),
)

#let render_fraction_bar(style, shaded, parts) = box(width: visual_width + 0.45cm, height: row_height - 0.25cm)[
  #align(center + horizon)[
    #if style == "stack" {
      stack(
        spacing: 0.08cm,
        fraction_row(parts, shaded, ccs_blue.lighten(45%), 0.5cm),
        fraction_row(parts, shaded, ccs_red.lighten(55%), 0.5cm),
      )
    } else {
      fraction_row(parts, shaded, ccs_blue.lighten(45%), 1.15cm)
    }
  ]
]

#let render_visual(visual) = {
  if visual.kind == "shape2d" {
    render_shape_2d(visual.shape)
  } else if visual.kind == "fraction_bar" {
    render_fraction_bar(visual.style, visual.shaded, visual.parts)
  } else if visual.kind == "solid3d" {
    render_solid_3d(visual.solid)
  } else {
    box(width: visual_width, height: row_height - 0.25cm)[]
  }
}

#let render_horizontal_question(section) = grid(
  columns: (number_width, 1fr),
  align: (left, center),
  column-gutter: 0.15cm,
)[
  #text(fill: ccs_ink, weight: "semibold")[#section.number)]
][
  #text[#section.item.question]
]

#let render_stacked_question(section, lines) = grid(
  columns: (number_width, 1fr),
  align: (left, top),
  column-gutter: 0.15cm,
)[
  #text(fill: ccs_ink, weight: "semibold")[#section.number)]
][
  #set text(font: ("Menlo", "DejaVu Sans Mono", "Courier New"))
  #for (idx, line_text) in lines.enumerate() {
    box(width: stack_width)[#align(right + top)[#text(line_text)]]
    if idx < lines.len() - 1 {
      linebreak()
    }
  }
]

#let render_item_row(section) = {
  let lines = section.item.question.split("\n")
  let prompt = if lines.len() > 1 {
    render_stacked_question(section, lines)
  } else {
    render_horizontal_question(section)
  }
  let prompt_with_visual = if section.item.visuals.len() > 0 {
    grid(columns: (visual_width, 1fr), column-gutter: 0.28cm, align: (left, center))[
      #render_visual(section.item.visuals.at(0))
    ][
      #prompt
    ]
  } else {
    prompt
  }

  grid(
    columns: (1fr, rule_width, answer_column_width),
    align: (left, center),
    column-gutter: 0.28cm,
  )[
    #box(height: row_height, width: 100%)[#align(left + horizon)[#prompt_with_visual]]
  ][
    #render_vertical_rule()
  ][
    #render_answer_box(section.number, section.item.answer)
  ]
}

#let render_custom_row(section) = grid(
  columns: (1fr, rule_width, answer_column_width),
  align: (left, center),
  column-gutter: 0.28cm,
)[
  #box(height: row_height, width: 100%)[
    #align(left + horizon)[#text(style: "italic")[#section.content]]
  ]
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
