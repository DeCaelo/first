use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let layout = &args[1];
    let tags = &args[2];
    let heading = &args[3];
    let filename = &args[4];

    let _new_file_contents = format!(
        "---
layout: {layout}
tags: {tags}
status: draft
---

# {heading}

"
    );

    let _ = File::create(filename).unwrap();
}
