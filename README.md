aaaa# nessusx
Fast and reliable rust implementation of xml to json parser for qualys scans.

:arrow_right: [ddt file](https://qualysguard.qg2.apps.qualys.eu/scan-1.dtd)

## How to use it

```bash
nessusx -x qualys_report.xml > out.json
[WRN] Use with caution. You are responsible for your actions.
[WRN] Developers assume no liability and are not responsible for any misuse or damage.
```

## ... or use nessusx in your projects

```rust
use nessusx::from_str;

fn main() {

    let file: String = std::fs::read_to_string(xml).unwrap();
    let scan: nessusx::Scan = nessusx::from_str(&file).unwrap();
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
