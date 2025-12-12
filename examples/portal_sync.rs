// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::core::Edit;
use xilem::view::{flex_col, label, portal};
use xilem::{WidgetView, WidgetViewSequence};

struct NonSyncAppState {
    receiver: std::sync::mpsc::Receiver<u32>,
}

fn app_logic(state: &mut NonSyncAppState) -> impl WidgetView<Edit<NonSyncAppState>> + use<> {
    flex_col((
        // The culprit
        portal(label("Hello")),
        // Other stuff
        label("Foo"),
        label("Foo"),
    ))
}

// error[E0277]: `std::sync::mpsc::Receiver<u32>` cannot be shared between threads safely
//    --> examples/portal_send.rs:12:46
//     |
//  12 |   fn app_logic(state: &mut NonSyncAppState) -> impl WidgetView<Edit<NonSyncAppState>> + use<> {
//     |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<u32>` cannot be shared between threads safely
//  13 | /     flex_col((
//  14 | |         // The culprit
//  15 | |         portal(label("Hello")),
// ...   |
//  18 | |         label("Foo"),
//  19 | |     ))
//     | |______- return type was inferred to be `Flex<(Portal<Label, _, _>, Label, ...), _, _>` here
//     |
//     = help: within `Flex<(Portal<Label, ..., ()>, ..., ...), ...>`, the trait `Sync` is not implemented for `std::sync::mpsc::Receiver<u32>`
// note: required because it appears within the type `NonSyncAppState`
//    --> examples/portal_send.rs:8:8
//     |
//   8 | struct NonSyncAppState {
//     |        ^^^^^^^^^^^^^^^
//     = note: required because it appears within the type `&'static mut NonSyncAppState`
//     = note: required because it appears within the type `(&'static mut NonSyncAppState, ())`
// note: required because it appears within the type `PhantomData<(&'static mut NonSyncAppState, ())>`
//    --> /home/olivier-faure/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/marker.rs:822:12
//     |
// 822 | pub struct PhantomData<T: PointeeSized>;
//     |            ^^^^^^^^^^^
// note: required because it appears within the type `Portal<Label, &mut NonSyncAppState, ()>`
//    --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/portal.rs:27:12
//     |
//  27 | pub struct Portal<V, State, Action> {
//     |            ^^^^^^
//     = note: required because it appears within the type `(Portal<Label, &mut ..., ()>, ..., ...)`
// note: required because it appears within the type `Flex<(Portal<Label, ..., ()>, ..., ...), ...>`
//    --> /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/flex.rs:132:12
//     |
// 132 | pub struct Flex<Seq, State, Action = ()> {
//     |            ^^^^
//     = note: required for `Flex<(Portal<Label, ..., ()>, ..., ...), ...>` to implement `WidgetView<&'static mut NonSyncAppState>`
