#[derive(Clone, Copy, Debug)]
pub struct ApiService {
    pub label: &'static str,
    pub credential_name: &'static str,
    pub credential_command_name: &'static str,
    pub authorization_hint: &'static str,
    pub accept_header: Option<&'static str>,
}
