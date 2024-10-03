#set text(font: "Berkeley Mono", size: 0.9em)
#set page(margin: 0.5in)

#set list(indent: 1em, marker: "*")

#let resume = yaml("cv.yml")

#let datetime-from-str(s) = {
  let RE = regex("^([1-2][0-9]{3})-([0-1][0-9])-([0-3][0-9])$")

  let caps = s.match(RE).captures.map(int)

  datetime(year: caps.at(0), month: caps.at(1), day: caps.at(2))
}

#let section-item(
  name,
  url: none,
  note: none,
  description: none,
  start: none,
  end: none,
  body: none,
) = {
  name = if url == none {
    name
  } else {
    [#underline(link(url, name))]
  }

  note = if note != none {
    [ | #note ]
  }

  description = if description != none {
    [ _ #description _ ]
  }

  let date = if start == none and end == none {
    none
  } else if start == none {
    panic("cannot specify only end")
  } else {
    let end = if end == none {
      "Present"
    } else {
      datetime-from-str(end).display("[month repr:short]. [year]")
    }

    [#datetime-from-str(start).display("[month repr:short]. [year]") - #end]
  }

  body = if type(body) == array {
    list(..body)
  } else {
    body
  }

  block(below: 1em)[#box[=== #name] #note #h(1fr) #date]
  description
  body
}

#let section(name, items, f) = {
  let items = items.map(f)

  block(below: 1.5em)[== #name]
  line(length: 100%)
  block(above: 1.5em, for item in items {
    section-item(item.remove("name"), ..item)
  })
}

= #text(size: 1.25em, resume.basics.name)

#resume.basics.label

#link("mailto:" + resume.basics.email, resume.basics.email) |
#link("https://" + resume.basics.url, resume.basics.url)

#resume.basics.profiles.map(p => link(p.url)[*\[#lower(p.network)\]* #p.username]).join(" | ")

#resume.basics.about

#block(above: 1em)

#section("Technical Skills", resume.skills, s => {
  (name: s.name, body: s.keywords.join(", "))
})

#block(above: 1em)
#section("Experience", resume.work, w => {
  (
    name: w.name,
    note: w.position,
    description: w.at("description", default: none),
    start: w.at("startDate"),
    end: w.at("endDate", default: none),
    body: w.highlights,
  )
})

#block(above: 1em)
#section("Personal Projects (Self-Hosted)", resume.projects, p => {
  (
    name: p.name,
    url: p.at("url", default: none),
    note: p.keywords.join(", "),
    description: p.at("description", default: none),
    body: p.highlights,
  )
})

#block(above: 1em)
#section("Education", resume.education, e => {
  (
    name: e.institution,
    note: "GPA: " + e.score,
    description: e.studyType + " in " + e.area,
    start: e.at("startDate"),
    end: e.at("endDate"),
  )
})

