# git-dirty
Script that recurses through the given directory finding any git repositories and checks whether it has any loose changes and if state of the repository is clean. That includes changes in the working directory, index, stashed changes and warns if there are ignored files.

## usage
```
git-dirty [path]
```

## installation
installing from crates.io
```
cargo install git-dirty
```
building from source
```
cargo install --path ./
```