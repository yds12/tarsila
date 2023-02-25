Backend library for [Tarsila](https://crates.io/crates/tarsila). This crate is
the core of an image editor focused on pixel art and spritesheets, whereas
`tarsila` is the GUI frontend of this editor. Other GUIs can be created reusing
this crate for all the actual image manipulation. Note that the focus of this
crate is not to be an image manipulation library (for that we leverage other
crates such as `image`), but a fully-functional editor core that keeps track of
things like layers, canvasses, and is capable of processing events of an image
editor, such as creating or changing a layer, drawing a line, etc.
