# How to contribute

## Prerequisites

Make sure that [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) are installed.

## Project layout

* `categorization.yml` — this file assigns system call names to their categories
* `README.md.j2` — this template defines how the system call categorization will be rendered
* `UNKNOWN-*.md.j2` — these templates define how the unknown system calls (system calls that are not present in the`categorization.yml`) will be rendered
* `README.md`, `UNKNOWN-*.md` — these documents are auto-generated by the `generate` command from the two sources of truth above
* `src/` — this directory is where the project tooling (which implements the `generate` command above) lives

## Fork the project

See [Fork a repository](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo) article in the GitHub Docs.

## Update categorizations

System call categorizations are stored in the `categorization.yml` and you can simply edit it with a text editor/IDE of your choice.

A typical system call addition might look like this:

```patch
diff --git a/categorization.yml b/categorization.yml
index 93d7a81..930a2dd 100644
--- a/categorization.yml
+++ b/categorization.yml
@@ -509,6 +509,10 @@ file-system:
           desc: change the name or location of a file (relative to a directory file descriptor)
         - name: renameatx_np
           desc: change the name or location of a file (relative to a directory file descriptor)
+        - name: clonefileat
+          desc: create a copy-on-write clone of files
+        - name: fclonefileat
+          desc: create a copy-on-write clone of files (referenced by a file descriptor)
         - name: truncate
           desc: truncate or extend a file to a specified length
         - name: ftruncate
```

Here are some rule of thumbs:

* if a category contains 10 or more system calls, it's probably a good idea to split it, unless the system calls are highly interdependent

## Re-generate the documents

Run the following command to generate the `README.md` and `UNKNOWN-*.md` files:

```shell
cargo run -- generate
```

This command will also fetch the new system call definitions, so don't be worried if some system calls that you didn't touch were updated in the `README.md` or `UNKNOWN-*.md`, that's for the good.

## Commit the changes and submit a PR

See [Creating a pull request from a fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork) article in the GitHub Docs.