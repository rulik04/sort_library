# Sort Library

This is a Rust library containing implementations of various sorting algorithms. You can use this library to sort slices of any comparable type.

## Usage

### 1. Download and Use Locally

To use this library locally, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/rulik04/sort_library.git <destination_folder_path>
    ```
    Replace <destination_folder_path> with the path to the folder where you want to clone the repository.
    sort_library and the project where you want to use this library must be in the same folder.

2. Use the sorting algorithms in your Rust project by importing the desired module:

    ```rust
    use sort_library::{insertion_sort, merge_sort, quick_sort, selection_sort};
    ```

3. ![img](https://github.com/rulik04/sort_library/blob/master/img/example1.png)

### 2. Using via GitHub in `dependencies`

Add the following line to your `Cargo.toml` file under `[dependencies]`:

    ```toml
    sort_library = { git = "https://github.com/rulik04/sort_library.git" }
    ```

Then you can import and use the sorting algorithms as mentioned in the previous section.

### 3. Add via crates.io

Add the library to your `Cargo.toml` file using the `cargo add` command:

```bash
cargo add sort_library
```

And then you can import and use the sorting algorithms as mentioned in the previous sections.
