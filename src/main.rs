use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let layout = &args[1];
    let tags = &args[2];
    let heading = &args[3];
    let _filename = &args[4];

    let new_file_contents = format!(
        "---
layout: {layout}
tags: {tags}
status: draft
---

# {heading}

"
    );

    dbg!(new_file_contents);
}
