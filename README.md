# Installing from a fresh new system: (ubuntu 22.04LTS)

These are the commands needed (for me) to run the font manager binaries 
from a freshly installed ubuntu 22.04LTS with minimum packages. All needed
packages and tools should be installed by the commands below.

### install main tools

``` 
sudo apt install git curl
```

### install rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

you might need to restart your shell to get the correct paths. Check if `cargo` seems to work.

### install compilation tools

```
sudo apt install gcc g++ pkg-config
```


### install the project's dependencies

```
sudo apt install libglib2.0-0 libglib2.0-dev libpango1.0-dev libgdk-pixbuf2.0-dev libgtk-4-dev

sudo apt install ttf-mscorefonts-installer
```

### install the project

```
git clone https://github.com/gosub-io/poc-font-manager.git
cd poc-font-manager
```

### build the project binaries:

````
cargo build --workspace

cargo run --bin display-fonts
````

If you have a GUI running, you can test the gtk applicaiton:

```
cargo run --bin gtk-test
```

