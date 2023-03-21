use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: T,
    pub code: u64,
    pub msg: String,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            code: 0,
            msg: "ok".to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct PageData<T: Serialize> {
    pub content: Vec<T>,
    pub total: u64,
    pub pages: u64,
}

#[derive(Serialize)]
pub struct PageResponse<T>
where
    T: Serialize,
{
    pub data: PageData<T>,
    pub code: u64,
    pub msg: String,
}

impl<T: Serialize> PageResponse<T> {
    pub fn new(data: Vec<T>, total: u64, pages: u64) -> PageResponse<T> {
        PageResponse {
            data: PageData {
                content: data,
                total,
                pages,
            },
            code: 0,
            msg: "ok".to_owned(),
        }
    }
}
