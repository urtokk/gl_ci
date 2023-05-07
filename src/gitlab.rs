use serde::Deserialize;

/// struct to hold gitlab ci environment variables
#[derive(Debug, Deserialize)]
struct GitlabCiEnv {
    pub variables: Vec<GitlabCiEnvVar>,
}

/// struct to hold gitlab ci environment variable
/// https://docs.gitlab.com/ee/ci/variables/predefined_variables.html
#[derive(Debug, Deserialize)]
pub struct GitlabCiEnvVar {
    pub key: String,
    pub value: String,
    pub environment_scope: String,
}

// function to get the gitlab ci environment variables
fn get_gitlab_ci_env(server: &str, project: &str, token: &str, environment: &str) -> Result<GitlabCiEnv> {
    let url = format!("{}/api/v4/projects/{}/variables", server, project);
    let client = reqwest::blocking::Client::new();
    let resp = client.get(&url)
        .header("PRIVATE-TOKEN", token)
        .send()?;
    let resp = resp.text()?;
    let resp: GitlabCiEnv = serde_json::from_str(&resp)?;
    // filter the variables by environment, matches * and environment
    let resp = resp.variables.into_iter()
        .filter(|v| v.environment_scope == "*" || v.environment_scope == environment)
        .collect();
    Ok(resp)
}
