use clap::{App, Arg};
use futures::future::join_all;
use osshkeys::PublicKey;
use std::collections::HashMap;
use std::result::Result;

async fn get_ssh_key_for_user_from_host(
    username: &str,
    host: &str,
) -> Result<Vec<PublicKey>, reqwest::Error> {
    let res = reqwest::get(format!("https://{}/{}.keys", host, username)).await?;

    if let Err(err) = res.error_for_status_ref() {
        if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(err);
            // return Err("User not found")
        }

        return Err(err);
    }

    let text = res.text().await?;
    let mut keys: Vec<PublicKey> = vec![];

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
                keys.push(key);
            }
            Err(err) => {
                println!("We were unable to parse a key from {}: {}", host, err);
            }
        }
    }

    Ok(keys)
}

fn main() {
    let matches = App::new("Get Keys")
        .version("0.0.1")
        .author("Riccardo Padovani <riccardo@rpadovani.com>")
        .about("Retrieve users' SSH and GPG keys from GitHub and Gitlab")
        .arg(
            Arg::new("username")
                .required(true)
                .takes_value(true)
                .about("Username to look-up"),
        )
        .arg(
            Arg::new("hosts")
                .required(false)
                .takes_value(true)
                .default_values(&["gitlab.com", "github.com"])
                .about("Which hosts should I target?"),
        )
        // .arg(Arg::new("gpg")
        //     .long("gpg-keys")
        //     .about("Retrieve GPG keys instead of SSH keys"))
        .get_matches();

    let hosts: Vec<_> = matches.values_of("hosts").unwrap_or_default().collect();
    let username = matches.value_of("username").unwrap();

    let mut futures = vec![];

    for host in hosts {
        futures.push(get_ssh_key_for_user_from_host(username, host))
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        //TODO: this is concurrent, not parallel, improve in future
        let results = join_all(futures).await;
        // We use an hash map to store the keys so we are able to have free deduplication between
        // different hosts
        let mut keys: HashMap<String, PublicKey> = HashMap::new();

        for result in results {
            match result {
                Ok(result) => {
                    for key in result {
                        keys.insert(key.to_string(), key);
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }

        for key in keys.values() {
            println!("{}", key);
        }
    });
}
