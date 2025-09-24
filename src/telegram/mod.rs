pub mod keyboard;
pub mod update;

use std::{collections::HashMap, sync::Arc};

use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

use crate::cells::{Cells, cell::groups::Groups};

#[derive(Clone, Debug)]
pub struct UserState {
    pub group: Groups,
}
#[derive(Clone, Debug)]
pub struct State {
    pub users: Arc<Mutex<HashMap<u64, UserState>>>,
    pub cells: Arc<Cells<'static>>,
    pub pool: Pool<Sqlite>,
}

pub type TGState = Arc<State>;
