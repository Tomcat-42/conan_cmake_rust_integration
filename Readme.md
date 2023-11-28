## Conan/CMake Rust integration example

This is a simple example project to show how to safely call C++ code from Rust
using [conan-rs](https://github.com/Devolutions/conan-rs) and
[autocxx](https://github.com/google/autocxx).

### Project Structure

![image](https://github.com/Tomcat-42/conan_cmake_rust_migration/assets/44649669/61ddd7f2-03a1-4a99-bf8c-2b449e533136)

#### cpp

The c++ targets uses [conan1](https://docs.conan.io/en/1.6/introduction.html) as the package manager and [CMake](https://cmake.org/). This is a very common setup in _enterprise_/_legacy_ c++ codebases.

In this example, we have a static library, that answers the ultimate question to _life, universe and everything_, and a command line interface to it:

![image](https://github.com/Tomcat-42/conan_cmake_rust_migration/assets/44649669/b9f0dbf0-690b-4b6d-ba96-2c9fd1d52953)

* Static lib:
```c++
#include <deep_thought/answer.hpp>

namespace deep_thought {
  int answer() {
    return 42;
  }
} // namespace deep_thought
```

* Cli:
```c++
#include <deep_thought/answer.hpp>

#include <chrono>
#include <iostream>
#include <thread>

auto main() -> int {
  auto task = []() {
    std::cout << "Thinking..." << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(10));
    std::cout << "The answer is " << deep_thought::answer() << std::endl;
  };

  std::thread thread(task);
  thread.join();
  return 0;
}
```

After building it, we have the following:

![image](https://github.com/Tomcat-42/conan_cmake_rust_migration/assets/44649669/cde44334-405a-4aa6-940b-11a0a8c3d124)

#### Rust

We can take advantage of the many application areas that Rust is more suited than C++ for extending the codebase. For example, [Writing C++ desktop applications is a very painful process](https://www.reddit.com/r/linuxmasterrace/comments/7xkcwo/why_does_everyone_hate_gtk/), so we can resource to the marvelous Rust [Tauri Framework](https://tauri.app/) for developing a good user experience, while maintaining the existing c++ code:

![image](https://github.com/Tomcat-42/conan_cmake_rust_migration/assets/44649669/dd72102e-4267-4bf4-8b6f-5d634da7afb3)

1. Here we use the [conan-rs](https://github.com/Devolutions/conan-rs) crate to abstract the deps instalation, building and packaging of our c++ artifacs. Also, we automatically generate Rust bindings "reading" the codebase header files using the wonderful google [autocxx](https://github.com/google/autocxx) crate. Finally, we link the c++ static lib to our Tauri binary to have a working application.
2. In the "Tauri backend"(in the `./src/app-rs/src/ext.rs` file) we actually tell _autocxx_ which c++ functions we want to generate code:
```rust
use autocxx::prelude::*;

include_cpp! {
    #include "deep_thought/answer.hpp"
    safety!(unsafe_ffi)
    generate!("deep_thought::answer")
}

pub fn answer() -> i32 {
    ffi::deep_thought::answer().into()
}
```

And in the main Tauri binary file we can enroll this function to be used as a Tauri command:

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_rs::ext;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn answer() -> String {
    ext::answer().to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![answer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. Finally, we can invoke this file in the Tauri frontend (`./src/app-rs/src/app.rs`file):

```rust
...
    let answer = {
        let answer_msg_rc = answer_msg_rc.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let answer_msg = answer_msg_rc.clone();

            spawn_local(async move {
                let new_msg = invoke("answer", JsValue::UNDEFINED)
                    .await
                    .as_string()
                    .unwrap();
                answer_msg.set(new_msg);
            });
        })
    };
...
```

With this, we have a fully fledged application, combining the strenghts of both Rust and C++:

![image](https://github.com/Tomcat-42/conan_cmake_rust_migration/assets/44649669/7dd7760d-431c-44f3-864f-8bf787ab5020)

## See more

For more details, read the full [article](https://tomcat0x42.me/conan_cmake_rust_migration).

## References

- [conan](https://docs.conan.io/en/1.6/introduction.html)
- [cmake](https://cmake.org/)
- [autocxx](https://github.com/google/autocxx)
- [conan-rs](https://github.com/Devolutions/conan-rs)
- [Tauri](https://tauri.app/)
- [cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
