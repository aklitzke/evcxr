// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use evcxr::{self, EvalContext};
use std::io;
use std::sync::mpsc;

fn send_output<T: io::Write + Send + 'static>(channel: mpsc::Receiver<String>, mut output: T) {
    std::thread::spawn(move || {
        while let Ok(line) = channel.recv() {
            if writeln!(output, "{}", line).is_err() {
                break;
            }
        }
    });
}

fn has_nightly_compiler() -> bool {
    use std::process;
    match process::Command::new("cargo")
        .arg("+nightly")
        .arg("help")
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()
    {
        Ok(exit_status) => exit_status.success(),
        Err(_) => false,
    }
}

fn main() -> Result<(), evcxr::Error> {
    evcxr::runtime_hook();
    if !has_nightly_compiler() {
        println!("print_performance_info: Nightly compiler is required.");
        // Exit with Ok status. We run this from Travis CI and don't want to
        // fail when it gets run on non-nightly configurations.
        return Ok(());
    }
    let (mut ctx, outputs) = EvalContext::new()?;
    send_output(outputs.stderr, io::stderr());
    ctx.set_time_passes(true);
    ctx.eval("println!(\"41\");")?;
    ctx.eval("println!(\"42\");")?;
    Ok(())
}