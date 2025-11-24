// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::core::Edit;
use xilem::view::{MainAxisAlignment, flex_col, portal, text_button};

struct AppState {
    candidates: Vec<String>,
}

fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<> {
    let list = state
        .candidates
        .iter()
        .map(|c| text_button(c.as_str(), |_: &mut AppState| {}))
        .collect::<Vec<_>>();

    portal(flex_col(list).main_axis_alignment(MainAxisAlignment::Start))
}

// error[E0700]: hidden type for `impl WidgetView<&'static mut AppState>` captures lifetime that does not appear in bounds
//   --> examples/borrowed_arg.rs:19:5
//    |
// 12 | fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<> {
//    |                     -------------     --------------------------------------- opaque type defined here
//    |                     |
//    |                     hidden type `xilem::view::Portal<xilem::view::Flex<Vec<xilem::view::Button<&'static mut AppState, (), impl (for<'a> Fn(<&'static mut AppState as ViewArgument>::Params<'a>, Option<PointerButton>) -> MessageResult<()>) + Send + Sync + 'static, xilem::view::Label>>, &'static mut AppState>, &'static mut AppState, ()>` captures the anonymous lifetime defined here
// ...
// 19 |     portal(flex_col(list).main_axis_alignment(MainAxisAlignment::Start))
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |
// help: add `'_` to the `use<...>` bound to explicitly capture it
//    |
// 12 | fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<'_> {
//    |                                                                             ++
