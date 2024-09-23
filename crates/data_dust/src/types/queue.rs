use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct QueueMessage<T> {
    pub message_type: String,
    pub payload: T,
}

impl<T: Serialize> Serialize for QueueMessage<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("QueueMessage", 2)?;
        state.serialize_field("message_type", &self.message_type)?;
        state.serialize_field("payload", &self.payload)?;
        state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for QueueMessage<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            MessageType,
            Payload,
        }

        struct QueueMessageVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for QueueMessageVisitor<T> {
            type Value = QueueMessage<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct QueueMessage")
            }

            fn visit_map<V>(self, mut map: V) -> Result<QueueMessage<T>, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut message_type = None;
                let mut payload = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::MessageType => {
                            if message_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("message_type"));
                            }
                            message_type = Some(map.next_value()?);
                        }
                        Field::Payload => {
                            if payload.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload = Some(map.next_value()?);
                        }
                    }
                }
                let message_type =
                    message_type.ok_or_else(|| serde::de::Error::missing_field("message_type"))?;
                let payload = payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;
                Ok(QueueMessage {
                    message_type,
                    payload,
                })
            }
        }

        const FIELDS: &[&str] = &["message_type", "payload"];
        deserializer.deserialize_struct(
            "QueueMessage",
            FIELDS,
            QueueMessageVisitor(std::marker::PhantomData),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionPayload {
    pub submission_id: i64,
    pub user_id: uuid::Uuid,
    pub problem_id: i32,
    pub language: String,
    pub contest_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContestPayload {
    pub contest_id: i32,
}
