/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::meta::{FromGodot, GodotConvert, GodotType};
use crate::registry::property::Var;
use godot_ffi::GodotNullableFfi;

pub struct OnEditor<T> {
    inner: Option<T>,
}

impl<T: GodotConvert> GodotConvert for OnEditor<T>
where
    // Option<T>: Var,
    // Option<T>: GodotConvert,
    // T::Via: GodotType,
    <T::Via as GodotType>::Ffi: GodotNullableFfi,
    // Option<T::Via>: GodotType,
{
    type Via = <Option<T> as GodotConvert>::Via;
}

impl<T: GodotConvert> Var for OnEditor<T>
where
    //     T::Via: GodotType,
    <T::Via as GodotType>::Ffi: GodotNullableFfi,
    //
    Option<T>: Var,
    //T: Var,
    // Option<T>: GodotConvert<Via = Option<T::Via>>,
{
    fn get_property(&self) -> Self::Via {
        self.inner.get_property()
        //self.inner.as_ref().map(Var::get_property)
    }

    fn set_property(&mut self, value: Self::Via) {
        // self.inner.set_property(value)
        todo!()
    }
}
