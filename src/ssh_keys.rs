use osshkeys::PublicKey;

pub(crate) async fn get_ssh_key_for_user_from_host(
    username: &str,
    host: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let res = reqwest::get(format!("https://{}/{}.keys", host, username)).await?;

    if let Err(err) = res.error_for_status_ref() {
        if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(err);
            // return Err("User not found")
        }

        return Err(err);
    }

    let text = res.text().await?;
    let mut keys = Vec::new();

    // In each page we retrieve there could be multiple keys, separated by a newline.
    // We iterate over all of them
    for raw_key in text.lines() {
        // Gitlab adds comments in the output, and they are not considered valid from osshkeys,
        // so we need to remove them. To do so, we split the string in a vector, we truncate the
        // vector, and we recreate a string from it.
        // TODO: there is a better way?

        let key = match raw_key.match_indices(' ').nth(1) {
            Some(idx) => &raw_key[0..idx.0],
            None => &raw_key,
        };

        match PublicKey::from_keystr(key) {
            Ok(key) => {
                keys.push(key.to_string());
            }
            Err(err) => {
                println!("We were unable to parse a key from {}: {}", host, err);
            }
        }
    }

    Ok(keys)
}
