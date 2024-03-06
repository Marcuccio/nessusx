<h1 align="center">
    <br>
    <img src="assets/nessusx_logo.png" width="200px" alt="GoAD">
    <br>
    Nessus X - The definitive parser
</h1>

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

## nessusx is also a library and can be used in your projects

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
