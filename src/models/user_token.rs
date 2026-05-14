use ormlite::Model;

#[derive(Model, Debug)]
#[ormlite(table = "UserToken")]
pub struct UserToken {
    pub id: String,
    pub token_hash: String,
    pub user_id: String,
    pub locked: bool,
}