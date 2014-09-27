use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use input;

use {
    RenderArgs,
    RenderEvent,
    UpdateArgs,
    UpdateEvent,
    GenericEvent,
};

/// Contains the different game events.
#[deriving(Clone, PartialEq, Show)]
pub enum Event<I = input::InputEvent> {
    /// Render graphics.
    Render(RenderArgs),
    /// Update physical state of the game.
    Update(UpdateArgs),
    /// Input event.
    Input(I),
}

impl<I: GenericEvent> GenericEvent for Event<I> {
    #[inline(always)]
    fn from_event(event_trait_id: TypeId, ev: &Any) -> Option<Event<I>> {
        let update = TypeId::of::<Box<UpdateEvent>>();
        let render = TypeId::of::<Box<RenderEvent>>();
        match event_trait_id {
            x if x == update => {
                match ev.downcast_ref::<UpdateArgs>() {
                    Some(args) => Some(Update(args.clone())),
                    None => fail!("Expected `UpdateArgs`")
                }
            }
            x if x == render => {
                match ev.downcast_ref::<RenderArgs>() {
                    Some(args) => Some(Render(args.clone())),
                    None => fail!("Expected `RenderArgs`")
                }
            }
            _ => {
                let input: Option<I> = GenericEvent::from_event(
                    event_trait_id, ev
                );
                match input {
                    Some(input) => Some(Input(input)),
                    None => None
                }
            }
        }
    }

    #[inline(always)]
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|) {
        let update = TypeId::of::<Box<UpdateEvent>>();
        let render = TypeId::of::<Box<RenderEvent>>();
        match event_trait_id {
            x if x == update => {
                match *self {
                    Update(ref args) => f(args as &Any),
                    _ => {}
                }
            }
            x if x == render => {
                match *self {
                    Render(ref args) => f(args as &Any),
                    _ => {}
                }
            }
            _ => {
                match *self {
                    Input(ref input) => input.with_event(event_trait_id, f),
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn test_event() {
    use assert_event_trait;
    use MouseCursorEvent;

    // Update.
    let ref e = UpdateEvent::from_update_args(&UpdateArgs { dt: 1.0 }).unwrap();
    assert_event_trait::<Event, Box<UpdateEvent>>(e);

    // Render.
    let ref e = RenderEvent::from_render_args(
        &RenderArgs { ext_dt: 1.0, width: 0, height: 0 }
    ).unwrap();
    assert_event_trait::<Event, Box<RenderEvent>>(e);

    let ref e = MouseCursorEvent::from_xy(1.0, 0.0).unwrap();
    assert_event_trait::<Event, Box<MouseCursorEvent>>(e);
}
