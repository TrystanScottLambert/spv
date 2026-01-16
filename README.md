# spv

This is a simple command line tool that will tell you what the 4MOST acronym "SPV" stands for and how far away it roughly is. 

## Installation
First get rust which is very easy. 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone this repo
```
git clone git@github.com:TrystanScottLambert/spv.git
```

Move into the directory and build the executable.
```
cd spv
cargo build --release
```

Finally move the compiled executable into your binary folder.  
```
sudo mv target/release/spv /usr/local/bin
```

## Usage
Now you can write `spv` in your terminal and get told what that stands for as well as how many weeks away it is.
