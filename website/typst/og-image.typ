#let header-height = 60pt
#let footer-height = 4pt

#set page(width: 600pt, height: 315pt, margin: 0pt)

// TODO: set sans serif font

#let render-header = {
  rect(width: 100%, height: header-height, {
    place(left + horizon, dx: 30pt, {
      text(size: 22pt, weight: "semibold")[CIfly]
    })
  })
}

// TODO: add svg logo here as well
#render-header

// TODO: replace by title variable -> truncate

#place(
  left + top,
  dy: 60pt,
  block(height: 100% - header-height - footer-height, inset: 35pt, clip: true, {
    block(text(size: 36pt, weight: "semibold", "Article Name"))
  })
)

// TODO: footer
