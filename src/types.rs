// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(super) type Throwable<T> = Result<T, Box<dyn std::error::Error>>;