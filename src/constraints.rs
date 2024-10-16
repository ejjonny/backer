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
    pub(crate) lower: Option<f32>,
    pub(crate) upper: Option<f32>,
}

impl Constraint {
    pub(crate) fn clamp(&self, value: f32) -> f32 {
        match (self.lower, self.upper) {
            (None, None) => value,
            (None, Some(upper)) => value.min(upper),
            (Some(lower), None) => value.max(lower),
            (Some(lower), Some(upper)) => value.clamp(lower, upper),
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
                    width: Constraint {
                        lower: Some(
                            amounts.leading + child.width.lower.unwrap_or(0.) + amounts.trailing,
                        ),
                        upper: child
                            .width
                            .upper
                            .map(|upper| upper + amounts.leading + amounts.trailing),
                    },
                    height: Constraint {
                        lower: Some(
                            amounts.top + child.height.lower.unwrap_or(0.) + amounts.bottom,
                        ),
                        upper: child
                            .height
                            .upper
                            .map(|upper| upper + amounts.top + amounts.bottom),
                    },
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
            NodeValue::Explicit { options, element } => element
                .constraints(allocations[0], state, ctx)
                .combine_equal_priority(SizeConstraints::from_size(
                    options.clone(),
                    allocations[0],
                    state,
                    ctx,
                )),
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
        Self {
            lower: None,
            upper: None,
        }
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
    pub(crate) fn combine_equal_priority(self, other: Self) -> Self {
        SizeConstraints {
            width: self.width.combine_equal_priority(other.width),
            height: self.height.combine_equal_priority(other.height),
            aspect: self.aspect.or(other.aspect),
        }
    }
}

impl Constraint {
    pub(crate) fn clamping(&self, value: f32) -> f32 {
        match (self.lower, self.upper) {
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
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        // In terms of upper constraints - no constraint is the biggest constraint
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        Constraint { lower, upper }
    }
    pub(crate) fn combine_equal_priority(self, other: Self) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        Constraint { lower, upper }
    }
    pub(crate) fn combine_sum(self, other: Self, spacing: f32) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(bound)) | (Some(bound), None) => Some(bound + spacing),
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        Constraint { lower, upper }
    }
}

impl SizeConstraints {
    pub(crate) fn from_size<A, B>(value: Size<A, B>, area: Area, a: &mut A, b: &mut B) -> Self {
        let mut initial = SizeConstraints {
            width: if value.width_min.is_some() || value.width_max.is_some() {
                Constraint {
                    lower: value.width_min,
                    upper: value.width_max,
                }
            } else {
                Constraint {
                    lower: None,
                    upper: None,
                }
            },
            height: if value.height_min.is_some() || value.height_max.is_some() {
                Constraint {
                    lower: value.height_min,
                    upper: value.height_max,
                }
            } else {
                Constraint {
                    lower: None,
                    upper: None,
                }
            },
            aspect: value.aspect,
        };
        if let Some(dynamic) = value.dynamic_height {
            let result = Some(initial.height.clamp(dynamic(area.width, a, b)));
            initial.height.lower = result;
            initial.height.upper = result;
        }
        if let Some(dynamic) = value.dynamic_width {
            let result = Some(initial.width.clamp(dynamic(area.height, a, b)));
            initial.width.lower = result;
            initial.width.upper = result;
        }
        initial
    }
}
