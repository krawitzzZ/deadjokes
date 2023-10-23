use std::num::NonZeroU64;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JokeQuery {
    pub p: Option<NonZeroU64>, // page
    pub b: Option<String>,     // body
}

impl Into<app::JokeQuery> for JokeQuery {
    fn into(self) -> app::JokeQuery {
        // pagination starts with page 0, so we subtract 1
        // e.g. ?p=1 => page: 0, ?p=32 => page: 31
        let page = self.p.map_or(0, |nzp| nzp.get() - 1);
        app::JokeQuery { page, body: self.b }
    }
}
