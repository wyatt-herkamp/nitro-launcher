use nl_core::instance::{InstanceType, Vanilla};
use serde::{Deserialize, Serialize};
pub type Instance = nl_core::instance::Instance<DynInstanceType>;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum DynInstanceType {
    Vanilla(Vanilla),
}
impl InstanceType for DynInstanceType {
    fn get_minecraft_version(&self) -> &str {
        match self {
            DynInstanceType::Vanilla(vanilla) => &vanilla.minecraft_version,
        }
    }

    fn components(&self) -> &[nl_core::Component] {
        todo!()
    }

    fn mods(&self) -> &[nl_core::Mod] {
        todo!()
    }

    fn type_name(&self) -> &str {
        match self {
            DynInstanceType::Vanilla(vanilla) => vanilla.type_name(),
        }
    }
}
