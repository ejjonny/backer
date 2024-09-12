# Backer

![rust](https://github.com/ejjonny/backer/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/backer.svg)](https://crates.io/crates/backer)
[![downloads](https://img.shields.io/crates/d/backer.svg)](https://crates.io/crates/backer)
[![license](https://img.shields.io/crates/l/backer.svg)](https://github.com/ejjonny/backer/blob/main/LICENSE)

A library for straight-forward UI layout.

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & would be most useful along with a GUI library that can do GUI things (like [macroquad](https://github.com/not-fl3/macroquad) or [egui](https://github.com/emilk/egui))._

### Features ✨

- Declarative API: The code should look like the structure it defines
- Minimal interface: No confusing overloads or magic, cascading effects
- Intuitive constraints: Backer adheres to & smoothly resolves size constraints with an advanced algorithm
- Performant: Layout can be comfortably recalculated every frame
- Easy integration & compatibility: Backer should work with just about any UI library with a bit of glue - so it isn't really fine-tuned for any specific UI solution.

This project intends to be a flexible layout tool & not much else.

## Preview

Backer relies on simple rules that can compose to create complex, flexible layouts.

![align](https://github.com/user-attachments/assets/ee335c64-64b1-430f-9b1b-5069e97ef3e7)
![column](https://github.com/user-attachments/assets/38984680-637f-447d-8358-e4df37158fa4)
![complex](https://github.com/user-attachments/assets/fa8eb97c-e02c-44ad-8f25-f99f52131dd3)
![stretched](https://github.com/user-attachments/assets/81fd3e70-a504-49c7-92b6-f4c6b05a5371)

<details>
<summary>See the code</summary>

```rust
    // #1
    row_spaced(
        10.,
        vec![
            // Custom draw nodes
            draw_a(ui),
            draw_b(ui).size(Size::new().height(150.).width(50.).y_align(YAlign::Bottom)),
            draw_c(ui),
        ],
    )
    .pad(10.)
    // #2
    column_spaced(
        10.,
        vec![
            draw_a(ui),
            draw_b(ui).size(
                Size::new()
                    .height(100.)
                    .width(150.)
                    .x_align(XAlign::Leading),
            ),
            draw_c(ui),
        ],
    )
    .pad(10.)
    // #3 & #4
    column_spaced(
        10.,
        vec![
            draw_a(ui),
            row_spaced(
                10.,
                vec![
                    draw_b(ui).size(Size::new().width(180.).x_align(XAlign::Leading)),
                    column_spaced(10., vec![draw_a(ui), draw_b(ui), draw_c(ui)]),
                ],
            ),
            draw_c(ui),
        ],
    )
    .pad(10.)
```

</details>

# Quick Start

## 1. Create a `Layout` struct with your layout function.

```rust
use backer::layout::Layout;
use backer::layout::Node;

let layout = Layout::new(my_layout_fn);

fn my_layout_fn(state: &mut MyState) -> Node<MyState> { todo!() }
```

## 2. Combine nodes to define your layout

```rust
fn my_layout_fn(state: &mut MyState) -> Node<MyState> {
  row(vec![
    ...
  ])
}
```

## 3. Implement a `draw` node

For reuse, you can construct your drawable in a function

```rust
fn my_drawable(state: &mut MyState) -> Node<MyState> {
  draw(move |area: Area, state: &mut MyState| {
    // The `area` parameter is the space alotted for your view after layout is calculated
    // The `state` parameter is *your* mutable state that you pass when you call layout.
    // This closure should draw UI based on the alotted area or update state so that drawing can be performed later.
  })
}
```

## 4. Run your layout

```rust
// UI libraries generally will expose methods to get the available screen size
// In a real implementation this should use the real screen size!
let available_area = Area {
        x: todo!(),
        y: todo!(),
        width: todo!(),
        height: todo!().
    };
let mut my_state = MyState::new();

let layout = Layout::new(my_layout_fn);
// Perform layout & draw all of your drawable nodes.
layout.draw(available_area, &mut my_state);
```

## Status

The crate is currently usable but is very new! Currently lacks extensive testing & will likely publish breaking changes often.

Contributions are always welcome 🤗
