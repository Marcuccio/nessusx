# nessusx
Fast and reliable rust implementation of xml to json parser for qualys scans.

## How to use it

```bash
# Default it serialize as csv in the stdout
nessusx file1.nessus file2.nessus
[WRN] Use with caution. You are responsible for your actions.

```
```bash
# ... or you can specify a path as output
nessusx --output tothisfile.csv file1.nessus file2.nessus
[WRN] Use with caution. You are responsible for your actions.

```
```bash
# ... and of course you can ask for a json output
nessusx --json --output tothisfile.csv file1.nessus file2.nessus
[WRN] Use with caution. You are responsible for your actions.

```

## ... or use nessusx in your projects

```rust
use nessusx::from_file;

fn main() {

    let scan: nessusx::Scan = nessusx::from_file(&path).unwrap();
    let j = serde_json::to_string(&scan).unwrap();
    
    println!("{}", j);
}
````
# Contribute

Contributions are always welcome! Please create a PR to add Github Profile.

## :pencil: License

This project is licensed under [GPL-3.0](https://opensource.org/license/gpl-3-0/) license.

## :man_astronaut: Show your support

Give a ⭐️ if this project helped you!
