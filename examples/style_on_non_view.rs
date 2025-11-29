// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::palette::css;
use xilem::view::{CrossAxisAlignment, FlexExt, label};

use xilem::style::Style as _;

fn some_box() -> impl WidgetView<()> {
    label("hello")
        .flex(CrossAxisAlignment::Start)
        .background_color(css::WHITE)
}

// error[E0599]: the method `background_color` exists for struct `FlexItem<xilem::view::Label, _, _>`, but its trait bounds were not satisfied
//    --> examples/style_on_non_view.rs:13:10
//     |
//  11 | /     label("hello")
//  12 | |         .flex(CrossAxisAlignment::Start)
//  13 | |         .background_color(css::WHITE)
//     | |         -^^^^^^^^^^^^^^^^ method cannot be called on `FlexItem<xilem::view::Label, _, _>` due to unsatisfied trait bounds
//     | |_________|
//     |
//     |
//    ::: /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/flex.rs:560:1
//     |
// 560 |   pub struct FlexItem<V, State, Action> {
//     |   ------------------------------------- doesn't satisfy `_: Style<_, _>` or `_: WidgetView<_, _>`
//     |
//     = note: the following trait bounds were not satisfied:
//             `FlexItem<xilem::view::Label, _, _>: WidgetView<_, _>`
//             which is required by `FlexItem<xilem::view::Label, _, _>: xilem::style::Style<_, _>`
//             `&FlexItem<xilem::view::Label, _, _>: WidgetView<_, _>`
//             which is required by `&FlexItem<xilem::view::Label, _, _>: xilem::style::Style<_, _>`
//             `&mut FlexItem<xilem::view::Label, _, _>: WidgetView<_, _>`
//             which is required by `&mut FlexItem<xilem::view::Label, _, _>: xilem::style::Style<_, _>`
