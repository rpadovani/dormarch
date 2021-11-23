mod gpg_keys;
mod ssh_keys;

use clap::{App, Arg};
use futures::future::join_all;
use futures::FutureExt;
use std::collections::HashSet;

use gpg_keys::get_gpg_key_for_user_from_host;
use ssh_keys::get_ssh_key_for_user_from_host;

fn main() {
    let matches = App::new("dormarch")
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
        .arg(
            Arg::new("gpg")
                .long("gpg-keys")
                .about("Retrieve GPG keys instead of SSH keys"),
        )
        .get_matches();

    let hosts: Vec<_> = matches.values_of("hosts").unwrap_or_default().collect();
    let username = matches.value_of("username").unwrap();
    let is_gpg = matches.is_present("gpg");

    let mut futures = vec![];

    for host in hosts {
        if is_gpg {
            futures.push(get_gpg_key_for_user_from_host(username, host).boxed());
        } else {
            futures.push(get_ssh_key_for_user_from_host(username, host).boxed());
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        //TODO: this is concurrent, not parallel, improve in future
        let results = join_all(futures).await;

        // We use an hash set to store the keys so we are able to have free deduplication between
        // different hosts (e.g., same key on GitLab.com and GitHub.com)
        let mut keys: HashSet<String> = HashSet::new();

        for result in results {
            match result {
                Ok(result) => {
                    for key in result {
                        keys.insert(key);
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }

        for key in keys.iter() {
            println!("{}", key);
        }
    });
}
