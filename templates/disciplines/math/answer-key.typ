#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(width: 29.7cm, height: 21cm, margin: 0.45cm)
#set text(size: spec.profile.customisations.layout.font_size * 0.82pt)
#set par(leading: 0.12em, spacing: 0em)

#let section_component(section) = {
  if section.type != "item" {
    "other"
  } else if section.item.question.starts-with("Language:") {
    "language"
  } else if section.item.question.starts-with("Symbols:") {
    "symbols"
  } else if section.item.question.starts-with("Diagrams:") {
    "diagrams"
  } else {
    "other"
  }
}

#let language_sections = spec.sections.filter(section => section_component(section) == "language")
#let symbols_sections = spec.sections.filter(section => section_component(section) == "symbols")
#let diagrams_sections = spec.sections.filter(section => section_component(section) == "diagrams")
#let other_sections = spec.sections.filter(section => section_component(section) == "other")
#let ordered_sections = language_sections + symbols_sections + diagrams_sections + other_sections

#let total = ordered_sections.len()
#let rows = 11
#let cols = 3
#let items_per_page = rows * cols
#let page_count = calc.ceil(total / items_per_page)
#let body_height = 18.95cm
#let row_height = body_height / rows
#let number_col_width = 1.3em
#let stack_width = 3.5cm

#let render_header() = [
  #grid(columns: (1fr, auto))[
    #text(size: spec.profile.customisations.layout.font_size * 1.0pt, weight: "bold")[Math Answer Key]
  ][
    #align(right)[
      #text(size: spec.profile.customisations.layout.font_size * 0.8pt)[Date: #spec.profile.customisations.header.date]
    ]
  ]
  #v(0.5cm)
]

#let render_stacked_with_answer(lines, answer) = block[
  #set text(font: "DejaVu Sans Mono")
  #for (idx, line) in lines.enumerate() {
    if idx < lines.len() - 1 {
      box(width: stack_width)[#align(right + top)[#text(line)]]
      linebreak()
    } else {
      box(width: stack_width)[#align(right + top)[#text(line + "  Ans: " + answer)]]
    }
  }
]

#let render_item(section) = {
  let lines = section.item.question.split("\n")
  let answer = section.item.answer
  let content = if lines.len() > 1 {
    render_stacked_with_answer(lines, answer)
  } else {
    text(lines.at(0) + "  Ans: " + answer)
  }

  grid(columns: (number_col_width, 1fr), gutter: 0.3em, align: (left, top))[
    #text(str(section.number) + ".")
  ][
    #content
  ]
}

#let has_component_sections = language_sections.len() + symbols_sections.len() + diagrams_sections.len() > 0

#let render_standard_pages(sections) = {
  let page_total = sections.len()
  let page_count = calc.ceil(page_total / items_per_page)

  let render_slot(global_idx) = {
    if global_idx < page_total {
      let section = sections.at(global_idx)
      if section.type == "item" {
        block(height: row_height)[#render_item(section)]
      } else {
        block(height: row_height)[#text(section.content)]
      }
    } else {
      block(height: row_height)[]
    }
  }

  let render_column(start) = block(height: body_height)[
    #for i in range(0, rows) {
      render_slot(start + i)
    }
  ]

  for page in range(0, page_count) {
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
}

#if has_component_sections {
  let component_rows = calc.max(language_sections.len(), calc.max(symbols_sections.len(), diagrams_sections.len()))
  let component_row_height = body_height / (component_rows + 1)

  let render_component_slot(items, idx) = {
    if idx < items.len() {
      block(height: component_row_height)[#render_item(items.at(idx))]
    } else {
      block(height: component_row_height)[]
    }
  }

  let render_component_column(title, items) = block(height: body_height)[
    #text(weight: "bold")[#title]
    #for i in range(0, component_rows) {
      render_component_slot(items, i)
    }
  ]

  render_header()
  grid(columns: (1fr, 1fr, 1fr), gutter: 0.6cm)[
    #render_component_column("Language", language_sections)
  ][
    #render_component_column("Symbols", symbols_sections)
  ][
    #render_component_column("Diagrams", diagrams_sections)
  ]

  if other_sections.len() > 0 {
    pagebreak()
    render_standard_pages(other_sections)
  }
} else {
  render_standard_pages(ordered_sections)
}
