// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::view::text_button;

struct Foo;
struct Bar;
struct Foobar {
    foo: Foo,
    bar: Bar,
}

fn widget() -> impl WidgetView<Foobar> {
    text_button("button", |_| unimplemented!())
}

// error[E0277]: the trait bound `Foobar: ViewArgument` is not satisfied
//   --> examples/wrong_ref.rs:15:5
//    |
// 15 |     text_button("button", |_| unimplemented!())
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ViewArgument` is not implemented for `Foobar`
//    |
// note: required by a bound in `text_button`
//   --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/button.rs:99:27
//    |
// 99 | pub fn text_button<State: ViewArgument, Action>(
//    |                           ^^^^^^^^^^^^ required by this bound in `text_button`
// help: consider borrowing here
//    |
// 15 |     &text_button("button", |_| unimplemented!())
//    |     +
// 15 |     &mut text_button("button", |_| unimplemented!())
//    |     ++++
//
// error[E0277]: the trait bound `Foobar: ViewArgument` is not satisfied
//   --> examples/wrong_ref.rs:15:27
//    |
// 15 |     text_button("button", |_| unimplemented!())
//    |                           ^^^ the trait `ViewArgument` is not implemented for `Foobar`
//    |
//    = help: the following other types implement trait `ViewArgument`:
//              &'static T
//              &'static mut T
//              ()
//              (T0, T1)
//              (T0, T1, T2)
//              (T0, T1, T2, T3)
//              (T0, T1, T2, T3, T4)
//              (T0, T1, T2, T3, T4, T5)
//            and 7 others
//
// error[E0277]: the trait bound `Foobar: ViewArgument` is not satisfied
//   --> examples/wrong_ref.rs:14:16
//    |
// 14 | fn widget() -> impl WidgetView<Foobar> {
//    |                ^^^^^^^^^^^^^^^^^^^^^^^ the trait `ViewArgument` is not implemented for `Foobar`
// 15 |     text_button("button", |_| unimplemented!())
//    |     ------------------------------------------- return type was inferred to be `Button<_, _, ..., ...>` here
//    |
//    = help: the following other types implement trait `ViewArgument`:
//              &'static T
//              &'static mut T
//              ()
//              (T0, T1)
//              (T0, T1, T2)
//              (T0, T1, T2, T3)
//              (T0, T1, T2, T3, T4)
//              (T0, T1, T2, T3, T4, T5)
//            and 7 others
//    = note: required for `Button<Foobar, (), ..., ...>` to implement `WidgetView<Foobar>`
//    = note: the full name for the type has been written to '/home/olivier-faure/Documents/xilem-error-messages/target/debug/examples/wrong_ref-14eb05e2a5baabf8.long-type-18224785288289274546.txt'
//    = note: consider using `--verbose` to print the full type name to the console
//
// error[E0277]: the trait bound `Foobar: ViewArgument` is not satisfied
//   --> examples/wrong_ref.rs:15:27
//    |
// 15 |     text_button("button", |_| unimplemented!())
//    |                           ^^^^^^^^^^^^^^^^^^^^ the trait `ViewArgument` is not implemented for `Foobar`
//    |
//    = help: the following other types implement trait `ViewArgument`:
//              &'static T
//              &'static mut T
//              ()
//              (T0, T1)
//              (T0, T1, T2)
//              (T0, T1, T2, T3)
//              (T0, T1, T2, T3, T4)
//              (T0, T1, T2, T3, T4, T5)
//            and 7 others
