use crate::{
    layout::NodeValue,
    models::{Area, Size},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SizeConstraints {
    pub(crate) width: Constraint,
    pub(crate) height: Constraint,
    pub(crate) aspect: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Constraint {
    lower: Option<f32>,
    upper: Option<f32>,
}

impl Constraint {
    pub(crate) fn new(lower: Option<f32>, upper: Option<f32>) -> Self {
        assert!(Self::check_constraints(lower, upper));
        Self { lower, upper }
    }
    pub(crate) fn get_lower(&self) -> Option<f32> {
        self.lower
    }
    pub(crate) fn set_lower(&self, value: Option<f32>) -> Option<f32> {
        assert!(Self::check_constraints(value, self.upper));
        self.lower
    }
    pub(crate) fn get_upper(&self) -> Option<f32> {
        self.upper
    }
    pub(crate) fn set_upper(&self, value: Option<f32>) -> Option<f32> {
        assert!(Self::check_constraints(self.lower, value));
        self.lower
    }
    pub(crate) fn clamp(&self, value: f32) -> f32 {
        match (self.lower, self.upper) {
            (None, None) => value,
            (None, Some(upper)) => value.min(upper),
            (Some(lower), None) => value.max(lower),
            (Some(lower), Some(upper)) => value.clamp(lower, upper),
        }
    }
    fn check_constraints(lower: Option<f32>, upper: Option<f32>) -> bool {
        if let (Some(lower_unwrapped), Some(upper_unwrapped)) = (lower, upper) {
            lower_unwrapped <= upper_unwrapped
        } else {
            true
        }
    }
}

impl<State, Ctx> NodeValue<State, Ctx> {
    pub(crate) fn constraints(
        &mut self,
        available_area: Area,
        state: &mut State,
        ctx: &mut Ctx,
    ) -> SizeConstraints {
        let contextual_aligns = self.contextual_aligns();
        let allocations = self.allocate_area(
            available_area,
            contextual_aligns.0,
            contextual_aligns.1,
            state,
            ctx,
        );
        match self {
            NodeValue::Padding { amounts, element } => {
                let child = element.constraints(allocations[0], state, ctx);
                SizeConstraints {
                    width: Constraint::new(
                        Some(
                            amounts.leading
                                + child.width.get_lower().unwrap_or(0.)
                                + amounts.trailing,
                        ),
                        // None,
                        child
                            .width
                            .get_upper()
                            .map(|upper| upper + amounts.leading + amounts.trailing),
                    ),
                    height: Constraint::new(
                        Some(amounts.top + child.height.get_lower().unwrap_or(0.) + amounts.bottom),
                        child
                            .height
                            .get_upper()
                            .map(|upper| upper + amounts.top + amounts.bottom),
                    ),
                    aspect: None,
                }
            }
            NodeValue::Column {
                ref mut elements,
                spacing,
                ..
            } => elements
                .iter_mut()
                .zip(allocations.iter())
                .fold(
                    Option::<SizeConstraints>::None,
                    |current, (element, allocated)| {
                        if let Some(current) = current {
                            Some(SizeConstraints {
                                width: current.width.combine_adjacent_priority(
                                    element.constraints(*allocated, state, ctx).width,
                                ),
                                height: current.height.combine_sum(
                                    element.constraints(*allocated, state, ctx).height,
                                    *spacing,
                                ),
                                aspect: None,
                            })
                        } else {
                            Some(element.constraints(*allocated, state, ctx))
                        }
                    },
                )
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Row {
                ref mut elements,
                spacing,
                ..
            } => elements
                .iter_mut()
                .zip(allocations.iter())
                .fold(
                    Option::<SizeConstraints>::None,
                    |current, (element, allocated)| {
                        if let Some(current) = current {
                            Some(SizeConstraints {
                                width: current.width.combine_sum(
                                    element.constraints(*allocated, state, ctx).width,
                                    *spacing,
                                ),
                                height: current.height.combine_adjacent_priority(
                                    element.constraints(*allocated, state, ctx).height,
                                ),
                                aspect: None,
                            })
                        } else {
                            Some(element.constraints(*allocated, state, ctx))
                        }
                    },
                )
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Stack(elements) => elements
                .iter_mut()
                .fold(Option::<SizeConstraints>::None, |current, element| {
                    if let Some(current) = current {
                        Some(current.combine_adjacent_priority(element.constraints(
                            allocations[0],
                            state,
                            ctx,
                        )))
                    } else {
                        Some(element.constraints(allocations[0], state, ctx))
                    }
                })
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Explicit { options, element } => {
                SizeConstraints::from_size(options.clone(), allocations[0], state, ctx)
                    .combine_explicit_with_child(element.constraints(allocations[0], state, ctx))
            }
            NodeValue::Offset { element, .. } => element.constraints(allocations[0], state, ctx),
            NodeValue::Scope { scoped } => scoped.constraints(allocations[0], state, ctx),
            NodeValue::Draw(_) | NodeValue::Space | NodeValue::AreaReader { .. } => {
                SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }
            }
            NodeValue::Coupled { element, .. } => element.constraints(allocations[0], state, ctx),
            NodeValue::Empty | NodeValue::Group(_) => unreachable!(),
        }
    }
}

impl Constraint {
    pub(crate) fn none() -> Self {
        Self::new(None, None)
    }
}

impl SizeConstraints {
    pub(crate) fn combine_adjacent_priority(self, other: Self) -> Self {
        SizeConstraints {
            width: self.width.combine_adjacent_priority(other.width),
            height: self.height.combine_adjacent_priority(other.height),
            aspect: None,
        }
    }
    pub(crate) fn combine_explicit_with_child(self, child: Self) -> Self {
        SizeConstraints {
            width: self.width.combine_explicit_with_child(child.width),
            height: self.height.combine_explicit_with_child(child.height),
            aspect: self.aspect.or(child.aspect),
        }
    }
}

impl Constraint {
    pub(crate) fn clamping(&self, value: f32) -> f32 {
        match (self.get_lower(), self.get_upper()) {
            (None, None) => value,
            (None, Some(upper)) => value.min(upper),
            (Some(lower), None) => value.max(lower),
            (Some(lower), Some(upper)) => value.clamp(lower, upper),
        }
    }
}

impl Constraint {
    pub(crate) fn combine_adjacent_priority(self, other: Self) -> Self {
        // This always takes the bigger bound
        let lower = match (self.get_lower(), other.get_lower()) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        // In terms of upper constraints - no constraint is the biggest constraint
        let upper = match (self.get_upper(), other.get_upper()) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        Constraint::new(lower, upper)
    }
    pub(crate) fn combine_explicit_with_child(self, child: Self) -> Self {
        // Child constraint is limited by parent constraint as it propogates up
        // The parent can be thought of as a wrapper which hides the constraints of it's child
        //
        // If there is no lower bound on the parent
        // & the lower bound on the child is higher than the parent's upper bound
        // we should limit the lower bound to the parent's upper bound
        //
        // The child can't override the parent
        Constraint::new(
            self.lower
                .or(child.lower.map(|cl| cl.min(self.upper.unwrap_or(cl)))),
            self.upper
                .or(child.upper.map(|cl| cl.max(self.lower.unwrap_or(cl)))),
        )
    }
    pub(crate) fn combine_sum(self, other: Self, spacing: f32) -> Self {
        let lower = match (self.get_lower(), other.get_lower()) {
            (None, None) => None,
            (None, Some(bound)) | (Some(bound), None) => Some(bound + spacing),
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        let upper = match (self.get_upper(), other.get_upper()) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        Constraint::new(lower, upper)
    }
}

impl SizeConstraints {
    pub(crate) fn from_size<A, B>(value: Size<A, B>, area: Area, a: &mut A, b: &mut B) -> Self {
        let initial = SizeConstraints {
            width: if value.width_min.is_some() || value.width_max.is_some() {
                Constraint::new(value.width_min, value.width_max)
            } else {
                Constraint::none()
            },
            height: if value.height_min.is_some() || value.height_max.is_some() {
                Constraint::new(value.height_min, value.height_max)
            } else {
                Constraint::none()
            },
            aspect: value.aspect,
        };
        if let Some(dynamic) = value.dynamic_height {
            let result = Some(initial.height.clamp(dynamic(area.width, a, b)));
            initial.height.set_lower(result);
            initial.height.set_upper(result);
        }
        if let Some(dynamic) = value.dynamic_width {
            let result = Some(initial.width.clamp(dynamic(area.height, a, b)));
            initial.width.set_lower(result);
            initial.width.set_upper(result);
        }
        initial
    }
}
