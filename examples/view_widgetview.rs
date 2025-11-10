// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::core::View;
use xilem::view::{flex_row, label};

fn get_view() -> impl View<()> {
    label("1")
}

fn take_widget_view() -> impl WidgetView<()> {
    flex_row(get_view())
}

// This isn't too bad.

// error[E0107]: trait takes 3 generic arguments but 1 generic argument was supplied
//   --> examples/view_widgetview.rs:8:23
//    |
//  8 | fn get_view() -> impl View<()> {
//    |                       ^^^^ -- supplied 1 generic argument
//    |                       |
//    |                       expected 3 generic arguments
//    |
// note: trait defined here, with 3 generic parameters: `State`, `Action`, `Context`
//   --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem_core/src/view.rs:53:11
//    |
// 53 | pub trait View<State: ViewArgument, Action, Context: ViewPathTracker>:
//    |           ^^^^ -----                ------  -------
// help: add missing generic arguments
//    |
//  8 | fn get_view() -> impl View<(), Action, Context> {
//    |                              +++++++++++++++++

fn main() {}
