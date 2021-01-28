use serde::Deserialize;

/// Fem input/output data properties
#[derive(Deserialize, Debug, Clone)]
pub struct Properties {
    #[serde(rename = "nodeID")]
    pub node_id: Option<Vec<u32>>,
    #[serde(rename = "csLabel")]
    pub cs_label: Option<String>,
    pub location: Option<Vec<f64>>,
    pub component: Option<Vec<i32>>,
    pub components: Option<Vec<f64>>,
    pub area: Option<Vec<f64>>,
}
/// Fem input/output data
#[derive(Deserialize, Debug, Clone)]
pub struct IOData {
    types: String,
    #[serde(rename = "exciteIDs")]
    excite_ids: Option<Vec<u32>>,
    pub descriptions: String,
    pub indices: Vec<u32>,
    pub properties: Properties,
}
/// Fem input/output 2 states: on or off
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum IO {
    On(IOData),
    Off(IOData),
}
impl IO {
    pub fn switch_off(self) -> Self {
        match self {
            IO::On(data) => IO::Off(data),
            IO::Off(_) => self
        }
    }
    pub fn switch_on(self) -> Self {
        match self {
            IO::Off(data) => IO::On(data),
            IO::On(_) => self
        }
    }
    pub fn is_on(&self) -> bool {
        match self {
            IO::On(_) => true,
            IO::Off(_) => false
        }
    }
}
