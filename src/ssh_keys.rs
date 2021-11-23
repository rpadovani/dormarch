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
    let mut keys: Vec<String> = vec![];

    // In each page we retrieve there could be multiple keys, separated by a newline.
    // We iterate over all of them
    for raw_key in text.lines() {
        // Gitlab adds comments in the output, and they are not considered valid from osshkeys,
        // so we need to remove them. To do so, we split the string in a vector, we truncate the
        // vector, and we recreate a string from it.
        // TODO: there is a better way?
        let mut split_key: Vec<&str> = raw_key.split_inclusive(" ").collect();
        split_key.truncate(2);
        let key_without_comments = &*split_key.into_iter().collect::<String>();

        let key = PublicKey::from_keystr(key_without_comments);
        match key {
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
