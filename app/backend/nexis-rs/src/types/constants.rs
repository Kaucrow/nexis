use crate::prelude::*;
use types::mongodb::{
    Clothes, Cpu, Food, Gpu, IsCollection, Keyboard, LibraryItem, Tech, TechOther
};

pub const SSS_COOKIE_NAME: &'static str = "session";

pub const STORE_COLLS: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| HashMap::from([
    ("cyberion", vec![ Tech::coll_name(), Cpu::coll_name(), Gpu::coll_name(), Keyboard::coll_name(), TechOther::coll_name() ]),
    ("savoro", vec![ Food::coll_name() ]),
    ("vesti", vec![ Clothes::coll_name() ]),
    ("readon", vec![ LibraryItem::coll_name() ]),
]));

pub struct SessionPublicToken {
    pub uuid_key: &'static str,
    pub user_id_key: &'static str,
    pub role_key: &'static str,
}

pub struct SessionDataToken {
    pub session_key: &'static str,
}

pub struct EmailToken {
    pub user_id_key: &'static str,
    pub email_key: &'static str,
}

pub struct RoleSelectPublicToken {
    pub roleselect_key: &'static str,
}

pub struct RoleSelectDataToken {
    pub user_key: &'static str,
    pub remember_me_key: &'static str,
}

pub const SSS_PUB_TK: SessionPublicToken = SessionPublicToken {
    uuid_key: "session_uuid",
    user_id_key: "user_id",
    role_key: "role",
};

pub const SSS_DATA_TK: SessionDataToken = SessionDataToken {
    session_key: "session",
};

pub const EMAIL_TK: EmailToken = EmailToken {
    user_id_key: "user_id",
    email_key: "email_key"
};

pub const ROLESEL_PUB_TK: RoleSelectPublicToken = RoleSelectPublicToken {
    roleselect_key: "roleselect_key"
};

pub const ROLESEL_DATA_TK: RoleSelectDataToken = RoleSelectDataToken {
    user_key: "user",
    remember_me_key: "remember_me"
};