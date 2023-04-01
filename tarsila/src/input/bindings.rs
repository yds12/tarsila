use super::{InputEvent, KeyboardKey, KeyboardModifier};
use crate::mouse::CursorType;
use crate::{Effect, UiEvent};
use lapix::{Direction, Event, Point, Tool};
use macroquad::prelude as mq;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeySpec {
    InputEvents(Vec<InputEvent>),
    FollowMouse(Vec<InputEvent>),
}

impl From<InputEvent> for KeySpec {
    fn from(e: InputEvent) -> Self {
        Self::InputEvents(vec![e])
    }
}

impl From<Vec<InputEvent>> for KeySpec {
    fn from(v: Vec<InputEvent>) -> Self {
        Self::InputEvents(v)
    }
}

impl KeySpec {
    // TODO: this works but it's incredibly unoptimized.
    pub fn matches(&self, events: &[InputEvent]) -> bool {
        match self {
            Self::InputEvents(es) => Self::is_subset(es, events) && Self::matches_mod(events, es),
            Self::FollowMouse(es) => {
                Self::is_subset(es, events)
                    && Self::matches_mod(events, es)
                    && events
                        .iter()
                        .any(|e| matches!(e, InputEvent::MouseRealMove(_)))
            }
        }
    }

    // Checks whether the two sets of events have the same keyboard modifier
    fn matches_mod(a: &[InputEvent], b: &[InputEvent]) -> bool {
        let mut a = a.iter().filter(|e| matches!(e, InputEvent::KeyModifier(_)));
        let mut b = b.iter().filter(|e| matches!(e, InputEvent::KeyModifier(_)));

        a.all(|e| b.any(|e2| e2 == e)) && b.all(|e| a.any(|e2| e2 == e))
    }

    fn is_subset(a: &[InputEvent], b: &[InputEvent]) -> bool {
        a.iter().all(|e| b.contains(e))
    }
}

type MouseAction = Box<dyn Fn(Point<i32>) -> Vec<Effect>>;

pub enum ActionSpec {
    Fx(Vec<Effect>),
    // TODO: by using a closure here, we cannot make this serializable, we
    // should later just have an enum of possible functions. Problem is that we
    // need to have the (Ui)Event parameterizable
    MouseFn(MouseAction),
}

impl From<Event> for ActionSpec {
    fn from(e: Event) -> Self {
        Self::Fx(vec![Effect::Event(e)])
    }
}

impl From<UiEvent> for ActionSpec {
    fn from(e: UiEvent) -> Self {
        Self::Fx(vec![Effect::UiEvent(e)])
    }
}

impl Debug for ActionSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Fx(vec) => f.write_fmt(format_args!("{:?}", vec)),
            Self::MouseFn(_) => f.write_str("MouseFn(f)"),
        }
    }
}

impl ActionSpec {
    pub fn produce(&self, events: &[InputEvent]) -> Vec<Effect> {
        match self {
            Self::Fx(fx) => fx.clone(),
            Self::MouseFn(f) => {
                for event in events {
                    if let InputEvent::MouseRealMove(delta) = event {
                        return (f)(*delta);
                    }
                }

                panic!("missing `MouseRealMove` event")
            }
        }
    }
}

#[derive(Debug)]
pub struct KeyBindings(Vec<(KeySpec, ActionSpec)>);

impl KeyBindings {
    pub fn new() -> Self {
        // TODO: bindings are very static, but sometimes we want to do something
        // more dynamic. E.g. when mouse moves, we want camera to move to the
        // same extent, not just in the same direction.
        let bindings = vec![
            // SET TOOLS
            (
                InputEvent::KeyPress(mq::KeyCode::B.into()).into(),
                Event::SetTool(Tool::Brush).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::G.into()).into(),
                Event::SetTool(Tool::Bucket).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::L.into()).into(),
                Event::SetTool(Tool::Line).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::R.into()).into(),
                Event::SetTool(Tool::Rectangle).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::I.into()).into(),
                Event::SetTool(Tool::Eyedropper).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::E.into()).into(),
                Event::SetTool(Tool::Eraser).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::S.into()).into(),
                Event::SetTool(Tool::Selection).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::M.into()).into(),
                Event::SetTool(Tool::Move).into(),
            ),
            // FLIP
            (
                InputEvent::KeyPress(mq::KeyCode::H.into()).into(),
                Event::FlipHorizontal.into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::V.into()).into(),
                Event::FlipVertical.into(),
            ),
            // ZOOM
            (
                vec![
                    InputEvent::MouseScrollUp,
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                ]
                .into(),
                UiEvent::ZoomAdd(0.25).into(),
            ),
            (
                vec![
                    InputEvent::MouseScrollDown,
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                ]
                .into(),
                UiEvent::ZoomAdd(-0.25).into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::Minus.into()).into(),
                UiEvent::ZoomOut.into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::Equal.into()).into(),
                UiEvent::ZoomIn.into(),
            ),
            // DRAWING
            (
                InputEvent::MouseButtonPress(mq::MouseButton::Left.into()).into(),
                UiEvent::ToolStart.into(),
            ),
            (
                InputEvent::MouseButtonDown(mq::MouseButton::Left.into()).into(),
                UiEvent::ToolStroke.into(),
            ),
            (
                InputEvent::MouseButtonRelease(mq::MouseButton::Left.into()).into(),
                UiEvent::ToolEnd.into(),
            ),
            // PAN CURSOR + CANVAS BLOCK
            (
                InputEvent::KeyPress(mq::KeyCode::Space.into()).into(),
                ActionSpec::Fx(vec![
                    Effect::UiEvent(UiEvent::BlockCanvas),
                    Effect::UiEvent(UiEvent::SetCursor(CursorType::Pan)),
                ]),
            ),
            (
                InputEvent::KeyRelease(mq::KeyCode::Space.into()).into(),
                ActionSpec::Fx(vec![
                    Effect::UiEvent(UiEvent::UnblockCanvas),
                    Effect::UiEvent(UiEvent::SetPreviousCursor),
                ]),
            ),
            // PAN
            (
                InputEvent::KeyDown(mq::KeyCode::Down.into()).into(),
                UiEvent::MoveCamera(Direction::Down).into(),
            ),
            (
                InputEvent::KeyDown(mq::KeyCode::Up.into()).into(),
                UiEvent::MoveCamera(Direction::Up).into(),
            ),
            (
                InputEvent::KeyDown(mq::KeyCode::Left.into()).into(),
                UiEvent::MoveCamera(Direction::Left).into(),
            ),
            (
                InputEvent::KeyDown(mq::KeyCode::Right.into()).into(),
                UiEvent::MoveCamera(Direction::Right).into(),
            ),
            (
                KeySpec::FollowMouse(vec![
                    InputEvent::KeyDown(mq::KeyCode::Space.into()),
                    InputEvent::MouseButtonDown(mq::MouseButton::Left.into()),
                ]),
                ActionSpec::MouseFn(Box::new(|p| {
                    vec![Effect::UiEvent(UiEvent::MoveCameraExact(Point::ZERO - p))]
                })),
            ),
            // UNDO
            (
                vec![
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                    InputEvent::KeyPress(mq::KeyCode::Z.into()),
                ]
                .into(),
                Event::Undo.into(),
            ),
            // COPY + PASTE
            (
                vec![
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                    InputEvent::KeyPress(mq::KeyCode::C.into()),
                ]
                .into(),
                Event::Copy.into(),
            ),
            (
                vec![
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                    InputEvent::KeyPress(mq::KeyCode::V.into()),
                ]
                .into(),
                UiEvent::Paste.into(),
            ),
            (
                InputEvent::KeyPress(mq::KeyCode::Delete.into()).into(),
                Event::DeleteSelection.into(),
            ),
        ];

        Self(bindings)
    }

    pub fn iter(&self) -> std::slice::Iter<(KeySpec, ActionSpec)> {
        self.0.iter()
    }

    pub fn used_keys(&self) -> Vec<KeyboardKey> {
        vec![
            mq::KeyCode::A.into(),
            mq::KeyCode::B.into(),
            mq::KeyCode::C.into(),
            mq::KeyCode::D.into(),
            mq::KeyCode::E.into(),
            mq::KeyCode::F.into(),
            mq::KeyCode::G.into(),
            mq::KeyCode::H.into(),
            mq::KeyCode::I.into(),
            mq::KeyCode::J.into(),
            mq::KeyCode::K.into(),
            mq::KeyCode::L.into(),
            mq::KeyCode::M.into(),
            mq::KeyCode::N.into(),
            mq::KeyCode::O.into(),
            mq::KeyCode::P.into(),
            mq::KeyCode::Q.into(),
            mq::KeyCode::R.into(),
            mq::KeyCode::S.into(),
            mq::KeyCode::T.into(),
            mq::KeyCode::U.into(),
            mq::KeyCode::V.into(),
            mq::KeyCode::W.into(),
            mq::KeyCode::X.into(),
            mq::KeyCode::Y.into(),
            mq::KeyCode::Z.into(),
            mq::KeyCode::Key0.into(),
            mq::KeyCode::Key1.into(),
            mq::KeyCode::Key2.into(),
            mq::KeyCode::Key3.into(),
            mq::KeyCode::Key4.into(),
            mq::KeyCode::Key5.into(),
            mq::KeyCode::Key6.into(),
            mq::KeyCode::Key7.into(),
            mq::KeyCode::Key8.into(),
            mq::KeyCode::Key9.into(),
            mq::KeyCode::Escape.into(),
            mq::KeyCode::Enter.into(),
            mq::KeyCode::Space.into(),
            mq::KeyCode::Minus.into(),
            mq::KeyCode::Equal.into(),
            mq::KeyCode::Up.into(),
            mq::KeyCode::Down.into(),
            mq::KeyCode::Left.into(),
            mq::KeyCode::Right.into(),
        ]
    }
}
