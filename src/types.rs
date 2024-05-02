// Copyright (C) 2024 Kawaxte
//
// This file is part of vrc-priority-rs.
//
// vrc-priority-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vrc-priority-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vrc-priority-rs.  If not, see <https://www.gnu.org/licenses/>.

pub(super) type Throwable<T> = Result<T, Box<dyn std::error::Error>>;
