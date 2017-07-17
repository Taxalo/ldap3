extern crate ldap3;

use std::error::Error;
use std::time::Duration;

use ldap3::{LdapConn, LdapConnBuilder, Scope, SearchEntry};

fn main() {
    match do_search() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

fn do_search() -> Result<(), Box<Error>> {
    let ldap = LdapConnBuilder::<LdapConn>::new()
        .with_conn_timeout(Duration::from_secs(5))
        .connect("ldap://localhost:2389")?;
    let (rs, res) = ldap
        .with_timeout(Duration::from_secs(5))
        .search(
            "ou=Places,dc=example,dc=org",
            Scope::Subtree,
            "(&(objectClass=locality)(l=man*))",
            vec!["l"]
        )?.success()?;
    println!("Result: {:?}", res);
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }
    Ok(())
}
