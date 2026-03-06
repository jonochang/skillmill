#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(width: 29.7cm, height: 21cm, margin: 0.45cm)
#set text(size: spec.profile.customisations.layout.font_size * 0.82pt)
#set par(leading: 0.12em, spacing: 0em)

#let total = spec.sections.len()
#let rows = 11
#let cols = 3
#let items_per_page = rows * cols
#let page_count = calc.ceil(total / items_per_page)
#let body_height = 18.95cm
#let row_height = body_height / rows
#let number_col_width = 1.3em
#let stack_width = 2.9cm

#let render_header() = [
  #grid(columns: (1fr, auto))[
    #text(size: spec.profile.customisations.layout.font_size * 1.0pt, weight: "bold")[Math Worksheet]
  ][
    #align(right)[
      #text(size: spec.profile.customisations.layout.font_size * 0.8pt)[Date: #spec.profile.customisations.header.date]
    ]
  ]
  #v(0.5cm)
]

#let render_stacked(lines) = block[
  #set text(font: "DejaVu Sans Mono")
  #for (idx, line) in lines.enumerate() {
    box(width: stack_width)[#align(right + top)[#text(line)]]
    if idx < lines.len() - 1 {
      linebreak()
    }
  }
]

#let render_item(section) = {
  let lines = section.item.question.split("\n")
  let content = if lines.len() > 1 {
    render_stacked(lines)
  } else {
    text(lines.at(0))
  }

  grid(columns: (number_col_width, 1fr), gutter: 0.3em, align: (left, top))[
    #text(str(section.number) + ".")
  ][
    #content
  ]
}

#let render_slot(global_idx) = {
  if global_idx < total {
    let section = spec.sections.at(global_idx)
    if section.type == "item" {
      block(height: row_height)[#render_item(section)]
    } else {
      block(height: row_height)[#text(section.content)]
    }
  } else {
    block(height: row_height)[]
  }
}

#let render_column(start) = block(height: body_height)[
  #for i in range(0, rows) {
    render_slot(start + i)
  }
]

#for page in range(0, page_count) {
  let page_start = page * items_per_page
  render_header()
  columns(cols, gutter: 0.6cm)[
    #render_column(page_start)
    #render_column(page_start + rows)
    #render_column(page_start + rows * 2)
  ]
  if page < page_count - 1 {
    pagebreak()
  }
}
