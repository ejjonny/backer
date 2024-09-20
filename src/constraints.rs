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

impl<State> NodeValue<State> {
    pub(crate) fn constraints(&self, available_area: Area) -> SizeConstraints {
        match self {
            NodeValue::Padding { amounts, element } => {
                element.constraints(available_area).combine_sum(
                    SizeConstraints {
                        width: Constraint {
                            lower: Some(amounts.trailing + amounts.leading),
                            upper: None,
                        },
                        height: Constraint {
                            lower: Some(amounts.bottom + amounts.top),
                            upper: None,
                        },
                        aspect: None,
                    },
                    0.,
                )
            }
            NodeValue::Column {
                elements, spacing, ..
            } => elements
                .iter()
                .fold(Option::<SizeConstraints>::None, |current, element| {
                    if let Some(current) = current {
                        Some(SizeConstraints {
                            width: current.width.combine_adjacent_priority(
                                element.constraints(available_area).width,
                            ),
                            height: current
                                .height
                                .combine_sum(element.constraints(available_area).height, *spacing),
                            aspect: None,
                        })
                    } else {
                        Some(element.constraints(available_area))
                    }
                })
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Row {
                elements, spacing, ..
            } => elements
                .iter()
                .fold(Option::<SizeConstraints>::None, |current, element| {
                    if let Some(current) = current {
                        Some(SizeConstraints {
                            width: current
                                .width
                                .combine_sum(element.constraints(available_area).width, *spacing),
                            height: current.height.combine_adjacent_priority(
                                element.constraints(available_area).height,
                            ),
                            aspect: None,
                        })
                    } else {
                        Some(element.constraints(available_area))
                    }
                })
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Stack(elements) => elements
                .iter()
                .fold(Option::<SizeConstraints>::None, |current, element| {
                    if let Some(current) = current {
                        Some(current.combine_adjacent_priority(element.constraints(available_area)))
                    } else {
                        Some(element.constraints(available_area))
                    }
                })
                .unwrap_or(SizeConstraints {
                    width: Constraint::none(),
                    height: Constraint::none(),
                    aspect: None,
                }),
            NodeValue::Explicit { options, element } => element
                .constraints(available_area)
                .combine_equal_priority(SizeConstraints::from(*options)),
            NodeValue::Offset { element, .. } => element.constraints(available_area),
            NodeValue::Scope { scoped, .. } => scoped.constraints(available_area),
            NodeValue::AreaReader { .. } => SizeConstraints {
                width: Constraint::none(),
                height: Constraint::none(),
                aspect: None,
            },
            NodeValue::Draw(_) | NodeValue::Space => SizeConstraints {
                width: Constraint::none(),
                height: Constraint::none(),
                aspect: None,
            },
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
    pub(crate) fn combine_sum(self, other: Self, spacing: f32) -> Self {
        SizeConstraints {
            width: self.width.combine_sum(other.width, spacing),
            height: self.height.combine_sum(other.height, spacing),
            aspect: None,
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

impl From<Size> for SizeConstraints {
    fn from(value: Size) -> Self {
        SizeConstraints {
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
        }
    }
}
