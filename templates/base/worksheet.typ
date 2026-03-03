#let data = json.decode(sys.inputs.data)
#let spec = data.spec

#set page(margin: 1.5cm)
#set text(size: spec.profile.customisations.layout.font_size * 1pt)

= Worksheet
#let school = spec.profile.customisations.header.school
#let class = spec.profile.customisations.header.class
*Student:* #spec.profile.name \
School: #if school == none { "" } else { school } \
Class: #if class == none { "" } else { class } \
Date: #spec.profile.customisations.header.date

#for section in spec.sections {
  if section.type == "item" {
    #text(str(section.number) + ". " + section.item.question)
    #linebreak()
  } else {
    #text(section.content)
    #linebreak()
  }
}
