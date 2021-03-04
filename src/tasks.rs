// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Root {
//     #[serde(rename = "task_id")]
//     pub task_id: i64,
//     #[serde(rename = "task_type")]
//     pub task_type: String,
//     pub interval: i64,
//     pub repeat: i64,
//     #[serde(rename = "task_details")]
//     pub task_details: TaskDetails,
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TaskDetails {
//     #[serde(rename = "task_subtype")]
//     pub task_subtype: String,
//     pub params: Params,
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Params {
//     pub directory: String,
// }

