#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(margin: 1.5cm)
#set text(size: spec.profile.customisations.layout.font_size * 1pt)

= Answer Key
*Student:* #spec.profile.name \
Date: #spec.profile.customisations.header.date

#for section in spec.sections {
  if section.type == "item" {
    #text(str(section.number) + ". " + section.item.question + "  Answer: " + section.item.answer)
    #linebreak()
  } else {
    #text(section.content)
    #linebreak()
  }
}
