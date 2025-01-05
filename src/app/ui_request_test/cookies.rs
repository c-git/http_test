use std::{fmt::Display, time::SystemTime};

use reqwest_cross::reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CookieDisplayMode {
    NameOnly,
    #[default]
    NameValue,
    Pretty,
    SingleLine,
}

#[derive(Debug, Clone)]
pub struct Cookie {
    /// The cookie's name.
    name: String,
    /// The cookie's value.
    value: String,
    /// The cookie's expiration, if any.
    expires: Option<String>,
    /// The cookie's maximum age, if any.
    max_age: Option<String>,
    /// The cookie's domain, if any.
    domain: Option<String>,
    /// The cookie's path domain, if any.
    path: Option<String>,
    /// Whether this cookie was marked Secure.
    secure: &'static str,
    /// Whether this cookie was marked HttpOnly.
    http_only: &'static str,
}

impl From<reqwest::cookie::Cookie<'_>> for Cookie {
    fn from(value: reqwest::cookie::Cookie<'_>) -> Self {
        Self {
            name: value.name().to_string(),
            value: value.value().to_string(),
            expires: value
                .expires()
                .map(|x| {
                    {
                        chrono::DateTime::from_timestamp(
                            x.duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs()
                                .try_into()
                                .unwrap_or_default(),
                            0,
                        )
                    }
                    .map(|t| t.to_string())
                })
                .unwrap_or(None),
            max_age: value.max_age().map(|x| x.as_secs().to_string()),
            domain: value.domain().map(|x| x.to_string()),
            path: value.path().map(|x| x.to_string()),
            secure: if value.secure() { "yes" } else { "no" },
            http_only: if value.http_only() { "yes" } else { "no" },
        }
    }
}

impl Cookie {
    pub fn display(&self, mode: CookieDisplayMode) -> String {
        match mode {
            CookieDisplayMode::NameOnly => self.name.clone(),
            CookieDisplayMode::NameValue => format!("{} = {}", self.name, self.value),
            CookieDisplayMode::Pretty => {
                let fields = self.non_name_fields();
                let mut result = self.name.clone();
                for field in fields {
                    result.push('\t');
                    result.push_str(&field);
                    result.push('\n');
                }
                result.push_str("---\n");
                result
            }
            CookieDisplayMode::SingleLine => {
                let mut fields = vec![self.name.clone()];
                let mut other_fields = self.non_name_fields();
                fields.append(&mut other_fields);
                fields.join(", ")
            }
        }
    }

    fn non_name_fields(&self) -> Vec<String> {
        let mut result = vec![];
        if let Some(x) = self.expires.as_ref() {
            result.push(format!("expires = {x}"));
        }
        if let Some(x) = self.max_age.as_ref() {
            result.push(format!("max_age = {x}"));
        }
        if let Some(x) = self.domain.as_ref() {
            result.push(format!("domain = {x}"));
        }
        if let Some(x) = self.path.as_ref() {
            result.push(format!("path = {x}"));
        }
        result.push(format!("secure = {}", self.secure));
        result.push(format!("http_only = {}", self.http_only));
        result.push(format!("value = {}", self.value));

        result
    }
}

impl Display for CookieDisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CookieDisplayMode::NameOnly => "Name Only",
                CookieDisplayMode::NameValue => "Name and Value",
                CookieDisplayMode::Pretty => "Pretty",
                CookieDisplayMode::SingleLine => "Single Line",
            }
        )
    }
}
