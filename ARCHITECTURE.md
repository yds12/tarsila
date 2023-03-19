# Architecture

## General

Tarsila is a pixel art and spritesheet editor written in Rust with
[macroquad](https://macroquad.rs/) as graphics backend and
[egui](https://www.egui.rs/) for GUI. The project consists of 2 crates:

* `tarsila`: the frontend GUI of the editor;
* `lapix`: the backend/core of the editor, where all interesting things happen.

Since this is my first project with a GUI in Rust, I was not sure if `egui` and
`macroquad` would be the right choice, also the GUI ecosystem is notorious for
being in constant change and rapid evolution. Because of that, from the start
all the core logic was put in the `lapix` crate, whereas the `tarsila` crate
serves as a frontend.

## Frontend-Backend Communication

Tarsila deals with egui and macroquad directly, being responsible for capturing
input events from the user, translating these events into higher-level
image-manipulation events and routing those to the core `lapix`. All the
communication between `tarsila` and `lapix` happens in this fashion:

* `tarsila` sends events (of type `lapix::Event`) to `lapix` via the
  `lapix::State::execute` method;
* `tarsila` queries the state (`lapix::State`) to figure out what changed.

The flow of information from `tarsila` to `lapix` is very limited and can only
happen in the form of events. This has its tradeoffs:

* we "sort of" have
  [event sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and all
  its pros and cons;
* a clean API that can be easily reused by other frontend implementations (e.g.
  if we decide to move from `egui` to something else);
* this simplicity can be a problem sometimes, since we cannot communicate
  complex events from `tarsila` to `lapix` -- we should strive to keep the
  `Event` type simple and small;
* this might be good for tests, since we can create tests that are simply a
  sequence of events, followed by asserting on the `State`.

## Frontend Lifecycle

The frontend has its own state, called `UiState`. This state contains the
`lapix::State`. The lifecycle of the application starts with the calls to the
`update` and `draw` methods on `UiState`. These calls happen ideally about 60
times per second (but it can be more or less depending on performance).

During the update, `UiState` will call the `sync` method on the GUI components.
The sync phase is basically a way for the `UiState` to tell the GUI about
changes in the state. Since `UiState` has both the `Gui` object and the
`lapix::State` object, and since we need a mutable reference on `Gui` to update
it and a shared reference on `State`, we would need these two references, but
this would violate borrowing rules. So we solve the issue by copying the parts
of the state that are relevant to the GUI and send it during the sync.

After sync, the next part of the update method is to call the `update` method on
the GUI components. The components will then use their own state (updated via
sync) to update their GUI (`egui`) elements.

The last step, the draw method, is where the canvas and all the updated UI
components from `egui` are drawn in the screen.

## The Backend

The backend starts by creating an instance of the `State` type. From then on,
the `execute` method will be called with an `Event` for every change we want to
make.

The `State` has, among other things, a `Layers` object, which is a collection of
`Layer`s, each of which contains one `Canvas`. The canvas contains an image that
can be manipulated.

The main trait in Tarsila is the `Bitmap` trait (that perhaps should be renamed
to raster or something like that), which represents a matrix of pixels, i.e. an
image. This is a trait because the concrete type of the image will be probably
determined by the frontend. In our case it's a wrapper around macroquad's
`Image` type. The main purpose of this trait is to allow `lapix` to create
images of any size, and easily get and set pixels in this image.

