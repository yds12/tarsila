use super::{InputEvent, KeyboardKey, KeyboardModifier};
use crate::mouse::CursorType;
use crate::{Effect, UiEvent};
use lapix::{Direction, Point, Event, Tool};
use macroquad::prelude as mq;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeySpec {
    InputEvents(Vec<InputEvent>),
    FollowMouse(Vec<InputEvent>),
}

impl KeySpec {
    pub fn matches(&self, events: &[InputEvent]) -> bool {
        match self {
            Self::InputEvents(es) => es.iter().all(|e| events.contains(e)),
            Self::FollowMouse(es) => {
                es.iter().all(|e| events.contains(e))
                    && events
                        .iter()
                        .any(|e| matches!(e, InputEvent::MouseRealMove(_)))
            }
        }
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
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::B.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Brush))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::G.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Bucket))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::L.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Line))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::R.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Rectangle))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::I.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Eyedropper))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::KeyPressed(mq::KeyCode::E.into()),
                ]),
                ActionSpec::Fx(vec![Effect::Event(Event::SetTool(Tool::Eraser))]),
            ),
            // ZOOM
            (
                KeySpec::InputEvents(vec![
                    InputEvent::MouseScrollUp,
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                ]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ZoomAdd(1.))]),
            ),
            (
                KeySpec::InputEvents(vec![
                    InputEvent::MouseScrollDown,
                    InputEvent::KeyModifier(KeyboardModifier::Control),
                ]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ZoomAdd(-1.))]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyPressed(mq::KeyCode::Minus.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ZoomOut)]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyPressed(mq::KeyCode::Equal.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ZoomIn)]),
            ),
            // DRAWING
            (
                KeySpec::InputEvents(vec![InputEvent::MouseButtonPressed(
                    mq::MouseButton::Left.into(),
                )]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ToolStart)]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::MouseButtonDown(
                    mq::MouseButton::Left.into(),
                )]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ToolStroke)]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::MouseButtonReleased(
                    mq::MouseButton::Left.into(),
                )]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::ToolEnd)]),
            ),
            // PAN SIDEFX
            (
                KeySpec::InputEvents(vec![InputEvent::KeyPressed(mq::KeyCode::Space.into())]),
                ActionSpec::Fx(vec![
                    Effect::UiEvent(UiEvent::BlockCanvas),
                    Effect::UiEvent(UiEvent::SetCursor(CursorType::Pan)),
                ]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyRelease(mq::KeyCode::Space.into())]),
                ActionSpec::Fx(vec![
                    Effect::UiEvent(UiEvent::UnblockCanvas),
                    Effect::UiEvent(UiEvent::SetPreviousCursor),
                ]),
            ),
            // PAN
            (
                KeySpec::InputEvents(vec![InputEvent::KeyDown(mq::KeyCode::Down.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::MoveCamera(Direction::Down))]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyDown(mq::KeyCode::Up.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::MoveCamera(Direction::Up))]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyDown(mq::KeyCode::Left.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::MoveCamera(Direction::Left))]),
            ),
            (
                KeySpec::InputEvents(vec![InputEvent::KeyDown(mq::KeyCode::Right.into())]),
                ActionSpec::Fx(vec![Effect::UiEvent(UiEvent::MoveCamera(Direction::Right))]),
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
