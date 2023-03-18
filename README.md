**NOTE: this project is a work in progress, if you want to use it, please save
your work as frequently as possible to avoid losing it.**

![alt text](https://github.com/yds12/tarsila/blob/master/docs/screenshot.png?raw=true)

Tarsila is a pixel art and spritesheet editor written in Rust with
[macroquad](https://macroquad.rs/) as graphics backend and
[egui](https://www.egui.rs/) for GUI. The project is consists of 3 crates:

* `tarsila`: the frontend GUI of the editor;
* `lapix`: the backend/core of the editor, where all interesting things happen;
* `egui-macroquad-fork`: a fork of
  [egui-macroquad](https://github.com/optozorax/egui-macroquad) to integrate
  `egui` and `macroquad`.

To learn more about the architecture take a look at
[ARCHITECTURE.md](ARCHITECTURE.md).

To contribute, take a look at [CONTRIBUTING.md](CONTRIBUTING.md).

## Getting Started

Check out our [installation instructions](docs/install.md).

To learn how to use, take a look at the [user guide](docs/user_guide.md).

## Known Issues

Have in mind that this project is a work in progress and might have a lot of
bugs, incomplete or missing features and suboptimal performance here and there.
Some of the main gaps currently are:

* Works as intended on Linux, but there are some compatibility issues with MacOS
  (file dialog window does not open), and status on Windows is unknown;
* No error handling, everything panics;
* There are a few unit tests, but integration tests are missing, and coverage is
  far from 100%;
* There is room to improve when it comes to performance

Visit our [issues page](https://github.com/yds12/tarsila/issues) for known
problems/bugs.

