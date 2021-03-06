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

#![deny(
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

extern crate clap;
#[macro_use] extern crate log;

extern crate libimagentryedit;
extern crate libimagerror;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::path::PathBuf;

use libimagentryedit::edit::*;
use libimagentryedit::error::EditError as EE;
use libimagerror::trace::MapErrTrace;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::storeid::IntoStoreId;

mod ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-edit",
                                    &version,
                                    "Edit store entries with $EDITOR",
                                    ui::build_ui);

    let mut entry = {
        let path = rt.cli()
            .value_of("entry")
            .unwrap(); // safe by clap

        let sid = PathBuf::from(path).into_storeid().map_err_trace_exit_unwrap(1);

        rt.store()
            .get(sid)
            .map_err_trace_exit_unwrap(1)
            .ok_or(EE::from(format!("Entry {} does not exist", path)))
            .map_err_trace_exit_unwrap(1)
    };

    if rt.cli().is_present("edit-header") {
        // TODO: support editing of header
        warn!("Editing header is not yet supported by imag-edit");
        ::std::process::exit(1);
    }

    let _ = entry
        .edit_content(&rt)
        .map_err_trace_exit_unwrap(1);
}

