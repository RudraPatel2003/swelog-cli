const GITHUB_REPOS_API_URL: &str = "https://api.github.com/repos/";

/// ```
/// use github::repository_name::get_repository_name_from_repository_url;
///
/// let repository_url = "https://api.github.com/repos/RudraPatel2003/swelog-cli";
///
/// let repository_name = get_repository_name_from_repository_url(repository_url);
///
/// assert_eq!(repository_name, "RudraPatel2003/swelog-cli");
/// ```
#[must_use]
pub fn get_repository_name_from_repository_url(repository_url: &str) -> String {
    let Some(repository_name) = repository_url.strip_prefix(GITHUB_REPOS_API_URL) else {
        return String::from("Unknown repository");
    };

    repository_name.to_string()
}
