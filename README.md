# Cryo
A secure file sharing cli written in Rust.


## Clap commands:

#### To upload a file, 

*Small note: If you're copy/pasting from cmd/terminal, make sure to copy path without newline or type file path manually.*

```
cargo run -- upload <file_path>
```

#### To check if a file exists, 
```
cargo run -- exists <client_link>
```

#### To download a specific file, 
```
cargo run -- download <client_link>
```

#### To delete any file, 
```
cargo run -- delete <client_link>
```

