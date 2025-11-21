// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::core::{Edit, map_state};
use xilem::view::text_button;

struct Foo;
struct Bar;
struct Foobar {
    foo: Foo,
    bar: Bar,
}

fn widget() -> impl WidgetView<Edit<Foobar>> {
    map_state(
        text_button("button", |_| unimplemented!()),
        |foobar: &mut Foobar, ()| &foobar.foo,
    )
}

// error[E0283]: type annotations needed for `<_ as ViewArgument>::Params<'_>`
//   --> examples/map_state.rs:18:9
//    |
// 18 |         |foobar: &mut Foobar, ()| &foobar.foo,
//    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
//    |
//    = note: cannot satisfy `_: ViewArgument`
//    = help: the following types implement trait `ViewArgument`:
//              &'static T
//              &'static mut T
//              ()
//              (T0, T1)
//              (T0, T1, T2)
//              (T0, T1, T2, T3)
//              (T0, T1, T2, T3, T4)
//              (T0, T1, T2, T3, T4, T5)
//            and 7 others
// note: required by a bound in `map_state`
//   --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem_core/src/views/map_state.rs:67:17
//    |
// 61 | pub fn map_state<ParentState, ChildState, Action, Context: ViewPathTracker, V, F>(
//    |        --------- required by a bound in this function
// ...
// 67 |     ChildState: ViewArgument,
//    |                 ^^^^^^^^^^^^ required by this bound in `map_state`
// help: try giving this closure an explicit return type
//    |
// 18 |         |foobar: &mut Foobar, ()| -> <ChildState as ViewArgument>::Params<'_> { &foobar.foo },
//    |                                   +++++++++++++++++++++++++++++++++++++++++++++             +
