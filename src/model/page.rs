use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Page<T> {
    datas: Vec<T>,
    next: bool,
    all_count: Option<usize>,
}

impl<T> Page<T> {
    pub fn new(datas: Vec<T>, next: bool) -> Self {
        Self {
            datas: datas,
            next: next,
            all_count: None,
        }
    }

    pub fn set_count(&mut self, count: usize) {
        self.all_count = Some(count);
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PageQueryDO<T> {
    pub item: Option<T>,
    // start from zero
    pub page_index: usize,
    pub page_size: usize,
    pub get_all_count: bool,
}

impl<T> Deref for PageQueryDO<T> {
    type Target = Option<T>;

    fn deref<'a>(&'a self) -> &'a Option<T> {
        &self.item
    }
}

impl<T> DerefMut for PageQueryDO<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Option<T> {
        &mut self.item
    }
}
