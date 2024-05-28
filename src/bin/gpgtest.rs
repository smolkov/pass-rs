use gpgme::{Context, Key, Protocol};

use std::io::{self, prelude::*};
fn main() -> anyhow::Result<()> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    // let keys = ctx
        // .find_keys("4134381D78A27BBC")?
        // .filter_map(|x| x.ok())
        // .filter(|k| k.can_encrypt())
        // .collect();

    let key = ctx.get_key("4134381D78A27BBC")?;
    println!(
        "Public id:{} fingerprint:{}",
        key.id().unwrap_or("?"),
        key.fingerprint().unwrap_or("err")
    );

    let mut plaintext = "Hello, World!";
    let mut output = Vec::new();

    ctx.encrypt(Some(&key), plaintext, &mut output)?;
    io::stdout().write_all(&output)?;

    let mut result = Vec::new();

    ctx.decrypt(&output, &mut result).expect("decrypt error");

	println!("\n---------------------------");
    io::stdout().write_all(&result)?;
	println!("\n---------------------------");
    println!("{:?}\n\n", key);

    // println!("{:?}", privkey);

    // let plaintext = std::fs::read_to_string("readme.md")?;
    // println!("{}", plaintext);
    // let mut output = vec![];
    // ctx.decrypt(&mut output, plaintext).map_err(|e| anyhow::anyhow!("decrypting failed: {e:?}"))?;

    // println!("Begin Output:");
    // std::io::stdout().write_all(&output)?;
    // println!("End Output.");
    Ok(())
}
