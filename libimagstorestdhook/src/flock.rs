use toml::Value;

use fs2::FileExt;

use libimagstore::hook::Hook;
use libimagstore::hook::accessor::HookDataAccessor as HDA;
use libimagstore::hook::accessor::HookDataAccessorProvider;
use libimagstore::hook::accessor::StoreIdAccessor;
use libimagstore::hook::accessor::MutableHookDataAccessor;
use libimagstore::hook::accessor::NonMutableHookDataAccessor;
use libimagstore::hook::result::HookResult;
use libimagstore::hook::error::{HookError, HookErrorKind, MapErrInto};
use libimagstore::storeid::StoreId;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

trait EntryFlock {
    fn lock(&self) -> HookResult<()>;
    fn unlock(&self) -> HookResult<()>;
}

impl EntryFlock for Entry {

    fn lock(&self) -> HookResult<()> {
        use std::fs::File;

        self.get_location()
            .clone()
            .into_pathbuf()
            .map_err_into(HookErrorKind::HookExecutionError)
            .and_then(|loc| File::open(loc).map_err_into(HookErrorKind::HookExecutionError))
            .and_then(|file| file.lock_exclusive().map_err_into(HookErrorKind::HookExecutionError))
    }

    fn unlock(&self) -> HookResult<()> {
        use std::fs::File;

        self.get_location()
            .clone()
            .into_pathbuf()
            .map_err_into(HookErrorKind::HookExecutionError)
            .and_then(|loc| File::open(loc).map_err_into(HookErrorKind::HookExecutionError))
            .and_then(|file| file.unlock().map_err_into(HookErrorKind::HookExecutionError))
    }

}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Action {
    Lock,
    Unlock
}

fn action_to_str(a: &Action) -> &'static str {
    match *a {
        Action::Lock   => "lock",
        Action::Unlock => "unlock",
    }
}

#[derive(Debug, Clone)]
pub struct FlockUpdateHook {
    action: Action,
}

impl FlockUpdateHook {

    pub fn new(action: Action) -> FlockUpdateHook {
        FlockUpdateHook {
            action: action,
        }
    }

}

impl Hook for FlockUpdateHook {

    fn name(&self) -> &'static str {
        "stdhook_flock_update"
    }

    fn set_config(&mut self, _: &Value) {
        () // We are not configurable here.
    }

}

impl HookDataAccessorProvider for FlockUpdateHook {

    fn accessor(&self) -> HDA {
        HDA::StoreIdAccess(self)
    }

}

impl StoreIdAccessor for FlockUpdateHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), id);
        Ok(())
    }

}

impl MutableHookDataAccessor for FlockUpdateHook {

    fn access_mut(&self, fle: &mut FileLockEntry) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), fle.get_location());
        fle.lock()
            .map_err(|e| HookError::new(HookErrorKind::HookExecutionError, Some(Box::new(e))))
            .map(|_| ())
    }

}

impl NonMutableHookDataAccessor for FlockUpdateHook {

    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        debug!("[FLOCK HOOK][{}] {:?}", action_to_str(&self.action), fle.get_location());
        fle.unlock()
            .map_err(|e| HookError::new(HookErrorKind::HookExecutionError, Some(Box::new(e))))
            .map(|_| ())
    }

}

