pub trait AppError {
    fn get_message(&self) -> String;
}

pub struct HashError {
    msg: String,
}

impl HashError {
    pub fn new() -> Box<HashError> {
        Box::from(HashError { msg: "Hash Error".to_string() })
    }
}

impl AppError for HashError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct DbInsertError {
    msg: String,
}

impl DbInsertError {
    pub fn new() -> Box<DbInsertError> {
        Box::from(DbInsertError { msg: "DB Insert Failed".to_string() })
    }
}

impl AppError for DbInsertError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct DbSelectError {
    msg: String,
}

impl DbSelectError {
    pub fn new() -> Box<DbSelectError> {
        Box::from(DbSelectError { msg: "DB Select Failed".to_string() })
    }
}
impl AppError for DbSelectError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct UsernameOrPasswordError {
    msg: String,
}

impl UsernameOrPasswordError {
    pub fn new() -> Box<UsernameOrPasswordError> {
        Box::from(UsernameOrPasswordError { msg: "Username or Password is incorrect".to_string() })
    }
}

impl AppError for UsernameOrPasswordError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct JwtError {
    msg: String,
}

impl JwtError {
    pub fn new() -> Box<JwtError> {
        Box::from(JwtError { msg: "JWT Error".to_string() })
    }
}

impl AppError for JwtError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct JsonError {
    msg: String,
}

impl JsonError {
    pub fn new() -> Box<JsonError> {
        Box::from(JsonError { msg: "JSON Error".to_string() })
    }
}

impl AppError for JsonError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct DbConnectionError {
    msg: String,
}

impl DbConnectionError {
    pub fn new() -> Box<DbConnectionError> {
        Box::from(DbConnectionError { msg: "DB Connection Error".to_string() })
    }
}

impl AppError for DbConnectionError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct DbUpdateError {
    pub msg: String,
}

impl DbUpdateError {
    pub fn new() -> Box<DbUpdateError> {
        Box::from(DbUpdateError { msg: "DB Update Error".to_string() })
    }
}

impl AppError for DbUpdateError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct DbTransactionError {
    pub msg: String,
}

impl DbTransactionError {
    pub fn new() -> Box<DbTransactionError> {
        Box::from(DbTransactionError { msg: "DB Transaction Error".to_string() })
    }
}

impl AppError for DbTransactionError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}

pub struct ConfigError {
    msg: String,
}

impl ConfigError {
    pub fn new() -> Box<ConfigError> {
        Box::from(ConfigError { msg: "Config Error".to_string() })
    }
}

impl AppError for ConfigError {
    fn get_message(&self) -> String {
        self.msg.clone()
    }
}