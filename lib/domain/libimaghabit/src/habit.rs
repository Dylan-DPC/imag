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

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;
use chrono::NaiveDateTime;
use chrono::Local;
use chrono::NaiveDate;

use error::HabitError as HE;
use error::HabitErrorKind as HEK;
use error::*;
use iter::HabitInstanceStoreIdIterator;
use util::date_to_string;
use util::IsHabitCheck;

use libimagentrylink::internal::InternalLinker;
use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreIdIterator;

/// A HabitTemplate is a "template" of a habit. A user may define a habit "Eat vegetable".
/// If the user ate a vegetable, she should create a HabitInstance from the Habit with the
/// appropriate date (and optionally a comment) set.
pub trait HabitTemplate : Sized {

    /// Create an instance from this habit template
    ///
    /// By default creates an instance with the name of the template, the current time and the
    /// current date and copies the comment from the template to the instance.
    ///
    /// It uses `Store::retrieve()` underneath. So if there is already an instance for the day
    /// passed, this will simply return the instance.
    fn create_instance_with_date<'a>(&self, store: &'a Store, date: &NaiveDate)
        -> Result<FileLockEntry<'a>>;

    /// Shortcut for calling `Self::create_instance_with_date()` with an instance of
    /// `::chrono::Local::today().naive_local()`.
    fn create_instance_today<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>>;

    /// Get instances for this template
    fn linked_instances(&self) -> Result<HabitInstanceStoreIdIterator>;

    /// Get the date of the next date when the habit should be done
    fn next_instance_date_after(&self, base: &NaiveDateTime) -> Result<Option<NaiveDate>>;

    /// Get the date of the next date when the habit should be done
    fn next_instance_date(&self) -> Result<Option<NaiveDate>>;

    /// Check whether the instance is a habit by checking its headers for the habit data
    fn is_habit_template(&self) -> Result<bool>;

    fn habit_name(&self) -> Result<String>;
    fn habit_basedate(&self) -> Result<String>;
    fn habit_recur_spec(&self) -> Result<String>;
    fn habit_comment(&self) -> Result<String>;
    fn habit_until_date(&self) -> Result<String>;

    /// Create a StoreId for a habit name and a date the habit should be instantiated for
    fn instance_id_for(habit_name: &String, habit_date: &NaiveDate) -> Result<StoreId>;
}

impl HabitTemplate for Entry {

    fn create_instance_with_date<'a>(&self, store: &'a Store, date: &NaiveDate) -> Result<FileLockEntry<'a>> {
        let name    = self.habit_name()?;
        let comment = self.habit_comment()?;
        let date    = date_to_string(date);
        let id      = instance_id_for_name_and_datestr(&name, &date)?;

        store.retrieve(id)
            .map_err(From::from)
            .and_then(|mut entry| {
                {
                    let mut hdr = entry.get_header_mut();
                    hdr.insert("habit.instance.name",    Value::String(name))?;
                    hdr.insert("habit.instance.date",    Value::String(date))?;
                    hdr.insert("habit.instance.comment", Value::String(comment))?;
                }
                Ok(entry)
            })
    }

    fn create_instance_today<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>> {
        self.create_instance_with_date(store, &Local::today().naive_local())
    }

    fn linked_instances(&self) -> Result<HabitInstanceStoreIdIterator> {
        let iter = self
            .get_internal_links()?
            .map(|link| link.get_store_id().clone())
            .filter(IsHabitCheck::is_habit_instance);

        let sidi = StoreIdIterator::new(Box::new(iter));
        Ok(HabitInstanceStoreIdIterator::new(sidi))
    }

    fn next_instance_date_after(&self, base: &NaiveDateTime) -> Result<Option<NaiveDate>> {
        use kairos::timetype::TimeType;
        use kairos::parser::parse;
        use kairos::parser::Parsed;
        use kairos::iter::extensions::Every;

        let date_from_s = |r: String| -> Result<TimeType> {
            match parse(&r)? {
                Parsed::TimeType(tt) => Ok(tt),
                Parsed::Iterator(_) => {
                    Err(format!("'{}' yields an iterator. Cannot use.", r).into())
                },
            }
        };

        debug!("Base is {:?}", base);

        let basedate  = date_from_s(self.habit_basedate()?)?;
        debug!("Basedate is {:?}", basedate);

        let increment = date_from_s(self.habit_recur_spec()?)?;
        debug!("Increment is {:?}", increment);

        let until = date_from_s(self.habit_until_date()?)?
            .calculate()?
            .get_moment()
            .map(Clone::clone)
            .ok_or_else(|| {
                let kind : HEK = "until-date seems to have non-date value".to_owned().into();
                HE::from_kind(kind)
            })?;
        debug!("Until-Date is {:?}", basedate);

        for element in basedate.every(increment)? {
            debug!("Calculating: {:?}", element);
            let element = element?.calculate()?;
            debug!(" = {:?}", element);
            if let Some(ndt) = element.get_moment() {
                if ndt >= base {
                    debug!("-> {:?} >= {:?}", ndt, base);
                    if ndt > &until {
                        return Ok(None);
                    } else {
                        return Ok(Some(ndt.date()));
                    }
                }
            } else {
                return Err("Iterator seems to return bogus values.".to_owned().into());
            }
        }

        unreachable!() // until we have habit-end-date support
    }

    /// Get the date of the next date when the habit should be done
    fn next_instance_date(&self) -> Result<Option<NaiveDate>> {
        use kairos::timetype::TimeType;

        let today = TimeType::today();
        let today = today.get_moment().unwrap(); // we know this is safe.
        debug!("Today is {:?}", today);

        self.next_instance_date_after(&today.date().and_hms(0, 0, 0))
    }

    /// Check whether the instance is a habit by checking its headers for the habit data
    fn is_habit_template(&self) -> Result<bool> {
        [
            "habit.template.name",
            "habit.template.basedate",
            "habit.template.comment",
        ].iter().fold(Ok(true), |acc, path| acc.and_then(|_| {
            self.get_header()
                .read(path)
                .map(|o| is_match!(o, Some(&Value::String(_))))
                .map_err(From::from)
        }))
    }

    fn habit_name(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.name")
    }

    fn habit_basedate(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.basedate")
    }

    fn habit_recur_spec(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.recurspec")
    }

    fn habit_comment(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.comment")
    }

    fn habit_until_date(&self) -> Result<String> {
        get_string_header_from_habit(self, "habit.template.until")
    }

    fn instance_id_for(habit_name: &String, habit_date: &NaiveDate) -> Result<StoreId> {
        instance_id_for_name_and_datestr(habit_name, &date_to_string(habit_date))
    }

}

fn instance_id_for_name_and_datestr(habit_name: &String, habit_date: &String) -> Result<StoreId> {
    use module_path::ModuleEntryPath;

    ModuleEntryPath::new(format!("instance/{}-{}", habit_name, habit_date))
        .into_storeid()
        .map_err(HE::from)
}

#[inline]
fn get_string_header_from_habit(e: &Entry, path: &'static str) -> Result<String> {
    match e.get_header().read(path)? {
        Some(&Value::String(ref s)) => Ok(s.clone()),
        Some(_) => Err(HEK::HeaderTypeError(path, "String").into()),
        None    => Err(HEK::HeaderFieldMissing(path).into()),
    }
}

pub mod builder {
    use toml::Value;
    use toml_query::insert::TomlValueInsertExt;
    use chrono::NaiveDate;

    use libimagstore::store::Store;
    use libimagstore::storeid::StoreId;
    use libimagstore::storeid::IntoStoreId;
    use libimagstore::store::FileLockEntry;

    use error::HabitError as HE;
    use error::HabitErrorKind as HEK;
    use error::*;
    use util::date_to_string;

    pub struct HabitBuilder {
        name: Option<String>,
        comment: Option<String>,
        basedate: Option<NaiveDate>,
        recurspec: Option<String>,
        untildate: Option<NaiveDate>,
    }

    impl HabitBuilder {

        pub fn with_name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }

        pub fn with_comment(mut self, comment: String) -> Self {
            self.comment = Some(comment);
            self
        }

        pub fn with_basedate(mut self, date: NaiveDate) -> Self {
            self.basedate = Some(date);
            self
        }

        pub fn with_recurspec(mut self, spec: String) -> Self {
            self.recurspec = Some(spec);
            self
        }

        pub fn with_until(mut self, date: NaiveDate) -> Self {
            self.untildate = Some(date);
            self
        }

        pub fn build<'a>(self, store: &'a Store) -> Result<FileLockEntry<'a>> {
            #[inline]
            fn mkerr(s: &'static str) -> HE {
                HE::from_kind(HEK::HabitBuilderMissing(s))
            }

            let name      = try!(self.name.ok_or_else(|| mkerr("name")));
            debug!("Success: Name present");

            let dateobj   = try!(self.basedate.ok_or_else(|| mkerr("date")));
            debug!("Success: Date present");

            let recur     = try!(self.recurspec.ok_or_else(|| mkerr("recurspec")));
            debug!("Success: Recurr spec present");

            let until     = try!(self.untildate.ok_or_else(|| mkerr("until-date")));
            debug!("Success: Until-Date present");

            if dateobj > until {
                let e = HE::from_kind(HEK::HabitBuilderLogicError("until-date before start date"));
                return Err(e);
            }

            if let Err(e) = ::kairos::parser::parse(&recur) {
                return Err(e).map_err(From::from);
            }
            let date      = date_to_string(&dateobj);
            let until     = date_to_string(&until);
            debug!("Success: Date valid");

            let comment   = self.comment.unwrap_or_else(|| String::new());
            let sid       = try!(build_habit_template_sid(&name));

            debug!("Creating entry in store for: {:?}", sid);
            let mut entry = try!(store.create(sid));

            try!(entry.get_header_mut().insert("habit.template.name", Value::String(name)));
            try!(entry.get_header_mut().insert("habit.template.basedate", Value::String(date)));
            try!(entry.get_header_mut().insert("habit.template.recurspec", Value::String(recur)));
            try!(entry.get_header_mut().insert("habit.template.comment", Value::String(comment)));
            try!(entry.get_header_mut().insert("habit.template.until", Value::String(until)));

            debug!("Success: Created entry in store and set headers");
            Ok(entry)
        }

    }

    impl Default for HabitBuilder {
        fn default() -> Self {
            HabitBuilder {
                name: None,
                comment: None,
                basedate: None,
                recurspec: None,
                untildate: None,
            }
        }
    }

    /// Buld a StoreId for a Habit from a date object and a name of a habit
    fn build_habit_template_sid(name: &String) -> Result<StoreId> {
        use module_path::ModuleEntryPath;
        ModuleEntryPath::new(format!("template/{}", name)).into_storeid().map_err(From::from)
    }

}

