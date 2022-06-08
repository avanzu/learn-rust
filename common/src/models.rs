use serde::Deserialize;


#[derive(Deserialize)]
pub struct Pager {
    pub limit: Option<i64>,
    pub skip: Option<i64>
}

impl Pager {

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50)
    }

    pub fn skip(&self) -> i64 {
        self.skip.unwrap_or(0)
    }
    
}