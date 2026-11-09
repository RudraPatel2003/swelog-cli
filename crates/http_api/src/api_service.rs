#[derive(Clone, Copy, Debug)]
pub struct ApiService {
    pub label: &'static str,
    pub credential: Option<CredentialDescription>,
    pub default_headers: &'static [(&'static str, &'static str)],
}

#[derive(Clone, Copy, Debug)]
pub struct CredentialDescription {
    pub name: &'static str,
    pub command_name: &'static str,
    pub authorization_hint: &'static str,
}
