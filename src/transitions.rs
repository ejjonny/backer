use crate::{models::Area, traits::Drawable};
use lilt::Animated;
pub use lilt::Easing;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

#[macro_export]
macro_rules! id {
    () => {{
        format!("{}:{}:{}", file!(), line!(), column!())
    }};
}

impl<State: TransitionState, T: TransitionDrawable<State>> Drawable<State> for T {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool) {
        let now = Instant::now();
        let mut hasher = DefaultHasher::new();
        self.id().hash(&mut hasher);
        let hsh = hasher.finish();
        let mut bank = state.bank().clone();
        let mut anim = bank.animations.remove(&hsh).unwrap_or(AnimArea {
            visible: Animated::new(visible)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            x: Animated::new(area.x)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            y: Animated::new(area.y)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            width: Animated::new(area.width)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            height: Animated::new(area.height)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
        });
        anim.visible.transition(visible, now);
        anim.x.transition(area.x, now);
        anim.y.transition(area.y, now);
        anim.width.transition(area.width, now);
        anim.height.transition(area.height, now);
        if visible || anim.visible.in_progress(now) {
            self.draw_interpolated(
                Area {
                    x: anim.x.animate_wrapped(now),
                    y: anim.y.animate_wrapped(now),
                    width: anim.width.animate_wrapped(now),
                    height: anim.height.animate_wrapped(now),
                },
                state,
                visible,
                anim.visible.animate_bool(0., 1., now),
            )
        }
        bank.animations.insert(hsh, anim);
        *state.bank() = bank;
    }
}
pub trait TransitionDrawable<State: TransitionState> {
    fn draw_interpolated(
        &mut self,
        area: Area,
        state: &mut State,
        visible: bool,
        visible_amount: f32,
    );
    fn id(&self) -> &u64;
    fn easing(&self) -> Easing;
    fn duration(&self) -> f32;
    fn delay(&self) -> f32;
}
pub trait TransitionState {
    fn bank(&mut self) -> &mut AnimationBank;
}
#[derive(Debug, Clone)]
pub struct AnimArea {
    visible: Animated<bool, Instant>,
    x: Animated<f32, Instant>,
    y: Animated<f32, Instant>,
    width: Animated<f32, Instant>,
    height: Animated<f32, Instant>,
}
#[derive(Debug, Clone)]
pub struct AnimationBank {
    animations: HashMap<u64, AnimArea>,
}
impl Default for AnimationBank {
    fn default() -> Self {
        Self::new()
    }
}
impl AnimationBank {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
    pub fn in_progress(&self, time: Instant) -> bool {
        for value in self.animations.values() {
            if value.visible.in_progress(time)
                || value.x.in_progress(time)
                || value.y.in_progress(time)
                || value.width.in_progress(time)
                || value.height.in_progress(time)
            {
                return true;
            }
        }
        false
    }
}
