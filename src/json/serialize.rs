use crate::rpc::activity::Activity;
use crate::rpc::packet::Packet;

use std::fmt::{Error, Write};

use super::utils::escape_json;

impl Packet<'_> {
    pub fn to_json(&self) -> Result<String, Error> {
        let mut json_str = String::new();

        json_str.push_str("{\"cmd\":\"SET_ACTIVITY\"");
        json_str.push_str(",\"nonce\":\"-\"");
        json_str.push_str(",\"args\":{");

        write!(&mut json_str, "\"pid\":{}", self.pid)?;
        if let Some(activity) = &self.activity {
            json_str.push_str(",\"activity\":");
            activity.push_json(&mut json_str)?;
        }

        json_str.push_str("}}");

        Ok(json_str)
    }
}

impl Activity<'_> {
    pub fn push_json(&self, json_str: &mut String) -> Result<(), Error> {
        write!(json_str, "{{\"type\":{}", self.ty.to_u8())?;

        if let Some(timestamps) = &self.timestamps {
            if timestamps.start.is_some() || timestamps.end.is_some() {
                json_str.push_str(",\"timestamps\":{");

                if let Some(start) = timestamps.start {
                    write!(json_str, "\"start\":{},", start)?;
                }

                if let Some(end) = timestamps.end {
                    write!(json_str, "\"end\":{}", end)?;
                }

                if json_str.ends_with(',') {
                    json_str.pop();
                }

                json_str.push('}');
            }
        }

        if let Some(details) = &self.details {
            write!(json_str, ",\"details\":\"{}\"", escape_json(details))?;
        }

        if let Some(state) = &self.state {
            write!(json_str, ",\"state\":\"{}\"", escape_json(state))?;
        }

        if let Some(party) = &self.party {
            json_str.push_str(",\"party\":{");

            if let Some(id) = &party.id {
                write!(json_str, "\"id\":\"{}\",", id)?;
            }

            if let Some(size) = &party.size {
                write!(json_str, "\"size\":[{},{}]", size[0], size[1])?;
            }

            if json_str.ends_with(',') {
                json_str.pop();
            }

            json_str.push('}');
        }

        if let Some(secrets) = &self.secrets {
            json_str.push_str(",\"secrets\":{");

            if let Some(join) = &secrets.join {
                write!(json_str, "\"join\":\"{}\",", join)?;
            }

            if let Some(spectate) = &secrets.spectate {
                write!(json_str, "\"spectate\":\"{}\",", spectate)?;
            }

            if let Some(match_id) = &secrets.match_id {
                write!(json_str, "\"match_id\":\"{}\"", match_id)?;
            }

            if json_str.ends_with(',') {
                json_str.pop();
            }

            json_str.push('}');
        }

        if let Some(assets) = &self.assets {
            json_str.push_str(",\"assets\":{");

            if let Some(large_image) = &assets.large_image {
                write!(json_str, "\"large_image\":\"{}\",", large_image)?;
            }

            if let Some(large_text) = &assets.large_text {
                write!(
                    json_str,
                    "\"large_text\":\"{}\",",
                    escape_json(large_text)
                )?;
            }

            if let Some(small_image) = &assets.small_image {
                write!(json_str, "\"small_image\":\"{}\",", small_image)?;
            }

            if let Some(small_text) = &assets.small_text {
                write!(
                    json_str,
                    "\"small_text\":\"{}\"",
                    escape_json(small_text)
                )?;
            }

            if json_str.ends_with(',') {
                json_str.pop();
            }

            json_str.push('}');
        }

        if let Some(buttons) = &self.buttons {
            json_str.push_str(",\"buttons\":[");

            for (index, button) in buttons.iter().enumerate() {
                if index > 0 {
                    json_str.push(',');
                }
                write!(
                    json_str,
                    "{{\"label\":\"{}\",\"url\":\"{}\"}}",
                    escape_json(button.label),
                    button.url
                )?;
            }

            json_str.push(']');
        }

        json_str.push('}');

        Ok(())
    }
}
