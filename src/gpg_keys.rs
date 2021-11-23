pub(crate) async fn get_gpg_key_for_user_from_host(
    username: &str,
    host: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let res = reqwest::get(format!("https://{}/{}.gpg", host, username)).await?;

    if let Err(err) = res.error_for_status_ref() {
        if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(err);
            // return Err("User not found")
        }

        return Err(err);
    }

    let text = res.text().await?;

    // GPG responses are quite something, we have to investigate how to make these working in a
    // sensible way
    Ok(vec![text])
}
