//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

#![recursion_limit="256"]

#![deny(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
extern crate itertools;
#[cfg(unix)] extern crate xdg_basedir;
extern crate env_logger;
extern crate ansi_term;
extern crate handlebars;

extern crate clap;
extern crate toml;
extern crate toml_query;
#[macro_use] extern crate is_match;

extern crate libimagstore;
extern crate libimagutil;
extern crate libimagerror;
extern crate libimaginteraction;

pub mod error;
pub mod configuration;
pub mod logger;
pub mod runtime;
pub mod setup;
pub mod spec;
pub mod version;

