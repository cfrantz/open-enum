// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate open_enum;
use open_enum::open_enum;

#[open_enum]
enum Color {
    Red,
    Blue,
    Green(u32),
}

#[open_enum]
enum Foo {
    Bar,
    Bin { field: u32 },
    Baz,
}


#[open_enum]
#[repr(u128)]
enum A {
    A,
    B,
    C,
}

#[open_enum]
#[repr(i128)]
enum B {
    A,
    B,
    C
}

#[open_enum]
enum C<T> {
    A,
    B,
    C
}

#[open_enum(foo)]
enum D {}

#[open_enum]
#[non_exhaustive]
enum E {
    A = 1,
}

fn main() {}
