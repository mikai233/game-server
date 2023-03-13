pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/com.youzu.got.protocol.rs"));
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use mlua::chunk;
    use mlua::prelude::LuaUserData;

    use crate::proto::{CsMessage, Language, LoginRequest};

    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let mut login = LoginRequest::default();
        login.account = "mikai233".into();
        login.server_id = "112233".into();
        login.language = Language::En.into();
        let json = serde_json::to_string(&login)?;
        println!("{}", json);
        let de_login: LoginRequest = serde_json::from_str(&json)?;
        assert_eq!(login, de_login);

        let mut msg_cs = CsMessage::default();
        msg_cs.login_request = Some(login);
        let json = serde_json::to_string(&msg_cs)?;
        println!("{}", json);
        let de_msg_cs = serde_json::from_str(&json)?;
        assert_eq!(msg_cs, de_msg_cs);
        Ok(())
    }

    #[test]
    fn test_serde_lua() -> anyhow::Result<()> {
        let lua = mlua::Lua::new();
        let mut login = LoginRequest::default();
        login.account = "mikai233".into();
        login.server_id = "112233".into();
        login.language = Language::En.into();
        let ud = lua.create_ser_userdata(login)?;
        lua.globals().set("login", ud)?;
        let c = chunk! {
            print(login)
        };
        lua.load(c).exec()?;
        Ok(())
    }

    impl LuaUserData for LoginRequest {}
}