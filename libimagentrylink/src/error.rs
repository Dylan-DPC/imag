//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

generate_error_module!(
    generate_error_types!(LinkError, LinkErrorKind,
        EntryHeaderReadError    => "Error while reading an entry header",
        EntryHeaderWriteError   => "Error while writing an entry header",
        ExistingLinkTypeWrong   => "Existing link entry has wrong type",
        LinkTargetDoesNotExist  => "Link target does not exist in the store",
        LinkParserError         => "Link cannot be parsed",
        LinkParserFieldMissingError => "Link cannot be parsed: Field missing",
        LinkParserFieldTypeError => "Link cannot be parsed: Field type wrong",
        InternalConversionError => "Error while converting values internally",
        InvalidUri              => "URI is not valid",
        StoreReadError          => "Store read error",
        StoreWriteError         => "Store write error",
        StoreIdError            => "StoreId handling error"
    );
);

pub use self::error::LinkError;
pub use self::error::LinkErrorKind;
pub use self::error::MapErrInto;

