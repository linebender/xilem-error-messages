// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::view::flex_row;
use xilem::{WidgetView, WidgetViewSequence};

fn returns_view_seq() -> impl WidgetViewSequence<()> {
    ()
}

fn my_component() -> impl WidgetView<()> {
    flex_row(returns_view_seq())
}

// error[E0277]: the trait bound `impl WidgetViewSequence<()>: FlexSequence<_, _>` is not satisfied
//    --> examples/flex_viewseq.rs:24:14
//     |
//  24 |     flex_row(returns_view_seq())
//     |     -------- ^^^^^^^^^^^^^^^^^^ the trait `View<_, _, ViewCtx>` is not implemented for `impl WidgetViewSequence<()>`
//     |     |
//     |     required by a bound introduced by this call
//     |
//     = help: the following other types implement trait `View<State, Action, Context>`:
//               `&'static str` implements `View<State, Action, Context>`
//               `(dyn AnyView<State, Action, Context, Element> + 'static)` implements `View<State, Action, Context>`
//               `dyn AnyView<State, Action, ..., ...> + Send` implements `View<State, Action, Context>`
//               `dyn AnyView<..., ..., ..., ...> + Send + Sync` implements `View<State, Action, Context>`
//               `dyn AnyView<State, Action, ..., ...> + Sync` implements `View<State, Action, Context>`
//               `AnyFlexChild<State, Action>` implements `View<State, Action, ViewCtx>`
//               `Box<V>` implements `View<State, Action, Context>`
//               `Cow<'static, str>` implements `View<State, Action, Context>`
//             and 55 others
//     = note: required for `impl WidgetViewSequence<()>` to implement `ViewSequence<_, _, ViewCtx, FlexElement>`
//     = note: required for `impl WidgetViewSequence<()>` to implement `FlexSequence<_, _>`
// note: required by a bound in `flex_row`
//    --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/flex.rs:110:51
//     |
// 110 | pub fn flex_row<State: ViewArgument, Action, Seq: FlexSequence<State, Action>>(
//     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `flex_row`
//     = note: the full name for the type has been written to '/home/olivier-faure/Documents/xilem-error-messages/target/debug/examples/flex_viewseq-ab19dbd930f5ba31.long-type-1411905975419512215.txt'
//     = note: consider using `--verbose` to print the full type name to the console

// error[E0277]: the trait bound `impl WidgetViewSequence<()>: View<(), (), ViewCtx>` is not satisfied
//   --> examples/flex_viewseq.rs:23:22
//    |
// 23 | fn my_component() -> impl WidgetView<()> {
//    |                      ^^^^^^^^^^^^^^^^^^^ the trait `View<(), (), ViewCtx>` is not implemented for `impl WidgetViewSequence<()>`
// 24 |     flex_row(returns_view_seq())
//    |     ---------------------------- return type was inferred to be `xilem::view::Flex<impl WidgetViewSequence<()>, _, _>` here
//    |
//    = help: the following other types implement trait `View<State, Action, Context>`:
//              `&'static str` implements `View<State, Action, Context>`
//              `(dyn AnyView<State, Action, Context, Element> + 'static)` implements `View<State, Action, Context>`
//              `dyn AnyView<State, Action, ..., ...> + Send` implements `View<State, Action, Context>`
//              `dyn AnyView<..., ..., ..., ...> + Send + Sync` implements `View<State, Action, Context>`
//              `dyn AnyView<State, Action, ..., ...> + Sync` implements `View<State, Action, Context>`
//              `AnyFlexChild<State, Action>` implements `View<State, Action, ViewCtx>`
//              `Box<V>` implements `View<State, Action, Context>`
//              `Cow<'static, str>` implements `View<State, Action, Context>`
//            and 55 others
//    = note: required for `impl WidgetViewSequence<()>` to implement `ViewSequence<(), (), ViewCtx, FlexElement>`
//    = note: required for `impl WidgetViewSequence<()>` to implement `FlexSequence<()>`
//    = note: required for `xilem::view::Flex<impl WidgetViewSequence<()>, ()>` to implement `View<(), (), ViewCtx>`
//    = note: required for `xilem::view::Flex<impl WidgetViewSequence<()>, ()>` to implement `WidgetView<()>`
//    = note: the full name for the type has been written to '/home/olivier-faure/Documents/xilem-error-messages/target/debug/examples/flex_viewseq-ab19dbd930f5ba31.long-type-1411905975419512215.txt'
//    = note: consider using `--verbose` to print the full type name to the console

// Some errors have detailed explanations: E0277, E0601.
// For more information about an error, try `rustc --explain E0277`.
// error: could not compile `xilem-error-messages` (example "flex_viewseq") due to 3 previous errors
