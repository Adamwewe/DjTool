# Rusty Music

EWA LETS GOOOOO

```rust

mod folder_crawler;
mod parse_path;

use folder_crawler::Crawler;
use parse_path::FolderParser;

let parsed = FolderParser::parser();
let items = Crawler::new(parsed.path)
    .crawl();

```