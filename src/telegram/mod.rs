pub mod keyboard;
pub mod update;

use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::cells::{Cells, cell::groups::Groups};

#[derive(Clone, Debug)]
pub struct UserState {
    pub group: Groups,
}
#[derive(Clone, Debug)]
pub struct State {
    pub cells: Arc<Cells<'static>>,
    pub pool: Pool<Sqlite>,
}

pub type TGState = Arc<State>;
