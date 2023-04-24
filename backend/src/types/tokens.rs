#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ConfirmationToken {
    pub user_id: uuid::Uuid,
}
