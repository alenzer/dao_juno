use crate::msg::ProjectInfo;
use cosmwasm_std::{Addr, Coin, DepsMut, StdResult, Uint64};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const OWNER: Item<Addr> = Item::new("owner");

pub const PROJECT_INFOS: Map<u64, ProjectInfo> = Map::new("project_infos");
