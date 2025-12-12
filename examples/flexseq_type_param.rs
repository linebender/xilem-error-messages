// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use xilem::WidgetView;
use xilem::style::Style as _;
use xilem::view::{FlexSequence, flex_row};

// This one is pretty treacherous. What the error *should* say is
// "Flex<...> can't implement WidgetView because
// there's no guarantee Seq implements Send + Sync".

fn take_flex_seq<Seq: FlexSequence<()>>(sequence: Seq) -> impl WidgetView<(), ()> {
    flex_row(sequence).padding(2.)
}

// error[E0599]: the method `padding` exists for struct `xilem::view::Flex<Seq, ()>`, but its trait bounds were not satisfied
//    --> examples/flexseq_type_param.rs:10:24
//     |
//  10 |     flex_row(sequence).padding(2.)
//     |                        ^^^^^^^ method cannot be called on `xilem::view::Flex<Seq, ()>` due to unsatisfied trait bounds
//     |
//    ::: /home/olivier-faure/.cargo/git/checkouts/xilem-420a6f61d3d10f60/7385e3d/xilem/src/view/flex.rs:132:1
//     |
// 132 | pub struct Flex<Seq, State, Action = ()> {
//     | ---------------------------------------- doesn't satisfy `_: Style<_, _>` or `xilem::view::Flex<Seq, ()>: WidgetView<_, _>`
//     |
//     = note: the following trait bounds were not satisfied:
//             `xilem::view::Flex<Seq, ()>: WidgetView<_, _>`
//             which is required by `xilem::view::Flex<Seq, ()>: xilem::style::Style<_, _>`
//             `&xilem::view::Flex<Seq, ()>: WidgetView<_, _>`
//             which is required by `&xilem::view::Flex<Seq, ()>: xilem::style::Style<_, _>`
//             `&mut xilem::view::Flex<Seq, ()>: WidgetView<_, _>`
//             which is required by `&mut xilem::view::Flex<Seq, ()>: xilem::style::Style<_, _>`
