# DJ Tool

Small track classification tool I am currently building for fun. Stack is in Rust (For crawling + melspectrogram generation) and Python (for ML). The Master branch is a few commits behind and not in use for now. Please refer to the baseline branch for current efforts in progress
```rust

mod folder_crawler;
mod parse_path;

use folder_crawler::Crawler;
use parse_path::FolderParser;

let parsed = FolderParser::parser();
let items = Crawler::new(parsed.path)
    .crawl();

```
