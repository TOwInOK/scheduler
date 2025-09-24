use crate::cells::cell::groups::Groups;

pub struct User {
    pub telegram_id: i32,
    pub selected_group: Groups,
}

impl User {
    pub fn new(telegram_id: i32, selected_group: Groups) -> Self {
        Self {
            telegram_id,
            selected_group,
        }
    }
}
