// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::core::one_of::OneOf4;
use xilem::view::{flex_col, label, prose};

struct Foobar;
struct NotFoobar;

fn component0() -> impl WidgetView<(), Foobar> {
    label("hello")
}

fn component1() -> impl WidgetView<(), Foobar> {
    label("hello")
}

fn component2() -> impl WidgetView<(), Foobar> {
    label("hello")
}

fn component_wrong() -> impl WidgetView<(), NotFoobar> {
    label("hello")
}

fn caller(n: u32) -> impl WidgetView<(), Foobar> {
    match n {
        0 => OneOf4::A(component0()),
        1 => OneOf4::B(component1()),
        2 => OneOf4::C(component2()),
        _ => OneOf4::D({
            flex_col((
                prose("Unknown media"),
                // component_wrong() has wrong Action type
                Some(component_wrong()),
            ))
        }),
    }
}

// error[E0277]: the trait bound `Flex<(..., ...), (), ...>: View<(), ..., ...>` is not satisfied
//   --> examples/wrong_action.rs:27:22
//    |
// 27 |   fn caller(n: u32) -> impl WidgetView<(), Foobar> {
//    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
// 28 | /     match n {
// 29 | |         0 => OneOf4::A(component0()),
// 30 | |         1 => OneOf4::B(component1()),
// 31 | |         2 => OneOf4::C(component2()),
// ...  |
// 38 | |         }),
// 39 | |     }
//    | |_____- return type was inferred to be `OneOf<..., ..., ..., ..., ..., ..., ..., ..., ...>` here
//    |
//    = help: the trait `View<(), Foobar, ViewCtx>` is not implemented for `Flex<(Prose<(), NotFoobar>, ...), (), ...>`
//            but trait `View<(), NotFoobar, ViewCtx>` is implemented for it
//    = help: for that trait implementation, expected `NotFoobar`, found `Foobar`
//    = note: required for `OneOf<..., ..., ..., ..., ..., ..., ..., ..., ...>` to implement `View<(), Foobar, ViewCtx>`
//    = note: required for `OneOf<..., ..., ..., ..., ..., ..., ..., ..., ...>` to implement `WidgetView<(), Foobar>`
//    = note: the full name for the type has been written to '/home/olivier-faure/Documents/xilem-error-messages/target/debug/examples/wrong_action-a91a0157203a2708.long-type-81428507681262448.txt'
//    = note: consider using `--verbose` to print the full type name to the console
