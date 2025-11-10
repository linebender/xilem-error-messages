// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::view::{flex_row, label};

fn returns_view_seq() -> impl WidgetView<()> {
    (
        label("1"),
        label("2"),
        label("Fizz"),
        label("4"),
        label("Buzz"),
        label("Fizz"),
        label("7"),
        label("8"),
        label("Fizz"),
        label("Buzz"),
    )
}

// This is what you should write instead:
fn returns_view() -> impl WidgetView<()> {
    flex_row((
        label("1"),
        label("2"),
        label("Fizz"),
        label("4"),
        label("Buzz"),
        label("Fizz"),
        label("7"),
        label("8"),
        label("Fizz"),
        label("Buzz"),
    ))
}

// error[E0277]: the trait bound `(..., ..., ..., ..., ..., ..., ..., ..., ..., ...): WidgetView<()>` is not satisfied
//   --> examples/view_viewseq.rs:9:26
//    |
//  9 |   fn returns_view_seq() -> impl WidgetView<()> {
//    |                            ^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
// 10 | /     (
// 11 | |         label("1"),
// 12 | |         label("2"),
// 13 | |         label("Fizz"),
// ...  |
// 20 | |         label("Buzz"),
// 21 | |     )
//    | |_____- return type was inferred to be `(..., ..., ..., ..., ..., ..., ..., ..., ..., ...)` here
//    |
//    = help: the trait `View<(), (), ViewCtx>` is not implemented for `(..., ..., ..., ..., ..., ..., ..., ..., ..., ...)`
//    = help: the following other types implement trait `View<State, Action, Context>`:
//              `&'static str` implements `View<State, Action, Context>`
//              `(dyn AnyView<State, Action, Context, Element> + 'static)` implements `View<State, Action, Context>`
//              `(dyn AnyView<State, Action, Context, Element> + Send + 'static)` implements `View<State, Action, Context>`
//              `(dyn AnyView<State, Action, Context, Element> + Send + Sync + 'static)` implements `View<State, Action, Context>`
//              `(dyn AnyView<State, Action, Context, Element> + Sync + 'static)` implements `View<State, Action, Context>`
//              `AnyFlexChild<State, Action>` implements `View<State, Action, ViewCtx>`
//              `Box<V>` implements `View<State, Action, Context>`
//              `Cow<'static, str>` implements `View<State, Action, Context>`
//            and 55 others
//    = note: required for `(..., ..., ..., ..., ..., ..., ..., ..., ..., ...)` to implement `WidgetView<()>`

fn main() {}
