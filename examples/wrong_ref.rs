// Copyright 2025 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::hint::black_box;

use xilem::WidgetView;
use xilem::core::MessageResult;
use xilem::core::map_message;
use xilem::core::{ViewArgument, map_state};
use xilem::view::{
    checkbox, flex_row, label, portal, progress_bar, prose, sized_box, slider, spinner, split,
    text_button, text_input,
};

struct Foo;
struct Bar;
struct Foobar {
    foo: Foo,
    bar: Bar,
}

fn widget() -> impl WidgetView<Foobar> {
    text_button("button", |_| unimplemented!())
}
