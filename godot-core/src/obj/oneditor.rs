/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::meta::{GodotConvert, GodotType};
use crate::registry::property::Var;

pub struct OnEditor<T> {
    inner: Option<T>,
}

impl<T: GodotConvert> GodotConvert for OnEditor<T>
where
    Option<T::Via>: GodotType,
{
    type Via = <Option<T> as GodotConvert>::Via;
}

impl<T: GodotConvert> Var for OnEditor<T>
where
    Option<T::Via>: GodotType,
    Option<T>: Var,
{
    fn get_property(&self) -> Self::Via {
        self.inner.get_property()
    }

    fn set_property(&mut self, value: Self::Via) {
        self.inner.set_property(value)
    }
}
