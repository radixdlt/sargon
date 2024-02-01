use crate::prelude::*;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::ops::AddAssign;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct IDGenerator<T: From<Uuid>> {
    ctr: Arc<Mutex<u64>>,
    phantom: PhantomData<T>,
}
impl<T: From<Uuid>> IDGenerator<T> {
    pub fn starting_at(ctr: u64) -> Self {
        Self {
            ctr: Arc::new(Mutex::new(ctr)),
            phantom: PhantomData,
        }
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::starting_at(0)
    }

    pub fn next(&self) -> T {
        let n = Uuid::from_u64_pair(0, **self.ctr.lock().unwrap().borrow());
        self.ctr.lock().unwrap().borrow_mut().add_assign(1);
        n.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn new_starts_at_zero() {
        assert_eq!(
            IDGenerator::<Uuid>::new().next().to_string(),
            "00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn consecutive() {
        let n = 0xff;
        let gen = IDGenerator::<Uuid>::new();
        let ids = (0..=n).map(|_| gen.next()).collect_vec();
        assert_eq!(
            ids.last().unwrap().to_string(),
            "00000000-0000-0000-0000-0000000000ff"
        );
    }
}
