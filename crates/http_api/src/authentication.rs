#[derive(Clone, Copy)]
pub enum Authentication<'secret> {
    Anonymous,
    BearerToken(&'secret str),
    ApiKeyHeader { header_name: &'static str, api_key: &'secret str },
}
