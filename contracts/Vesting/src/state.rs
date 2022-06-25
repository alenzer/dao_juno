use crate::msg::ProjectInfo;
use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};



pub const OWNER: Item<Addr> = Item::new("owner");

pub const PROJECT_INFOS: Map<u64, ProjectInfo> = Map::new("project_infos");
