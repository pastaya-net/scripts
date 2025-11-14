#!/usr/bin/env cargo +nightly -Zscript

const GIT_WORKSPACE: &str = env!("PASTAYA_NET_GIT_WORKSPACE");
const LICENSE_HEADER: &str = r#"
/**
 *
 * @licstart  The following is the entire license notice for the
 *  JavaScript code in this page.
 *
 * Copyright (C) 2025 pastaya
 *
 *
 * The JavaScript code in this page is free software: you can
 * redistribute it and/or modify it under the terms of the GNU
 * General Public License (GNU GPL) as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option)
 * any later version.  The code is distributed WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU GPL for more details.
 *
 * As additional permission under GNU GPL version 3 section 7, you
 * may distribute non-source (e.g., minimized or compacted) forms of
 * that code without the copy of the GNU GPL normally required by
 * section 4, provided you include this license notice and a URL
 * through which recipients can access the Corresponding Source.
 *
 * @licend  The above is the entire license notice
 * for the JavaScript code in this page.
 *
 */
"#;

fn main() -> std::io::Result<()> {
    for entry in std::fs::read_dir(format!("{GIT_WORKSPACE}/counterrs/pkg"))? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|extension| extension.to_str()) == Some("js") {
            let content = std::fs::read_to_string(&path)?;
            if content.contains("Copyright (C) 2025") {
                println!("license header found in {path:#?}, nevermind.");
            } else {
                let new_content = format!("{}\n{}", LICENSE_HEADER, content);
                std::fs::write(&path, new_content)?;
                println!("added license header to {:?}", path);
            }
        }
    }
    Ok(())
}
