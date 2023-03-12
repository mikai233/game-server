use std::fmt::Formatter;
use std::marker::PhantomData;

use protobuf::{EnumFull, EnumOrUnknown};

include!(concat!(env!("OUT_DIR"), "/game_proto/mod.rs"));

fn serialize_enum_or_unknown<E: EnumFull, S: serde::Serializer>(
    e: &EnumOrUnknown<E>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match e.enum_value() {
        Ok(v) => s.serialize_str(v.descriptor().name()),
        Err(v) => s.serialize_i32(v),
    }
}

fn deserialize_enum_or_unknown<'de, E: EnumFull, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<EnumOrUnknown<E>, D::Error> {
    struct DeserializeEnumVisitor<E: EnumFull>(PhantomData<E>);

    impl<'de, E: EnumFull> serde::de::Visitor<'de> for DeserializeEnumVisitor<E> {
        type Value = EnumOrUnknown<E>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "a string, an integer or none")
        }

        fn visit_i32<R>(self, v: i32) -> Result<Self::Value, R>
            where
                R: serde::de::Error,
        {
            Ok(EnumOrUnknown::from_i32(v))
        }

        fn visit_str<R>(self, v: &str) -> Result<Self::Value, R>
            where
                R: serde::de::Error,
        {
            match E::enum_descriptor().value_by_name(v) {
                Some(v) => Ok(EnumOrUnknown::from_i32(v.value())),
                None => Err(serde::de::Error::custom(format!(
                    "unknown enum value: {}",
                    v
                ))),
            }
        }

        fn visit_unit<R>(self) -> Result<Self::Value, R>
            where
                R: serde::de::Error,
        {
            Ok(EnumOrUnknown::default())
        }
    }

    d.deserialize_any(DeserializeEnumVisitor(PhantomData))
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(remote = "protobuf::MessageField")]
struct MessageFieldDef<T>(
    pub Option<Box<T>>
);

#[cfg(test)]
mod test {
    use protobuf::{EnumOrUnknown, MessageField};

    use crate::msg_types_cs::CSMessage;
    use crate::proto_common::Language;
    use crate::proto_login::LoginRequest;

    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let mut login = LoginRequest::default();
        login.account = "mikai233".into();
        login.server_id = "112233".into();
        login.language = EnumOrUnknown::new(Language::EN);
        let json = serde_json::to_string(&login)?;
        println!("{}", json);
        let de_login: LoginRequest = serde_json::from_str(&json)?;
        assert_eq!(login, de_login);

        let mut msg_cs = CSMessage::default();
        msg_cs.login_request = MessageField::some(login);
        let json = serde_json::to_string(&msg_cs)?;
        println!("{}", json);
        let de_msg_cs = serde_json::from_str(&json)?;
        assert_eq!(msg_cs, de_msg_cs);
        Ok(())
    }
}