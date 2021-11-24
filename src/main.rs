mod gpg_keys;
mod ssh_keys;

use clap::Parser;
use futures::future::join_all;
use futures::FutureExt;
use std::collections::HashSet;

use gpg_keys::get_gpg_key_for_user_from_host;
use ssh_keys::get_ssh_key_for_user_from_host;

/// Retrieve users' SSH and GPG keys from GitHub and Gitlab
#[derive(Parser)]
#[clap(
    version = "0.0.1",
    author = "Riccardo Padovani <riccardo@rpadovani.com>"
)]
struct Arguments {
    /// Username to look-up
    username: String,
    /// Which hosts should I target?
    #[clap(default_values = &["gitlab.com", "github.com"])]
    hosts: Vec<String>,
    /// Retrieve GPG keys instead of SSH keys
    #[clap(long = "gpg-keys")]
    is_gpg: bool,
}

#[tokio::main]
async fn main() {
    let args: Arguments = Arguments::parse();

    let mut futures = Vec::new();

    for host in &args.hosts {
        if args.is_gpg {
            futures.push(get_gpg_key_for_user_from_host(&args.username, host).boxed());
        } else {
            futures.push(get_ssh_key_for_user_from_host(&args.username, host).boxed());
        }
    }

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
}
