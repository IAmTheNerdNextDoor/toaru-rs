# toaru-rs

toaru-rs is a project aimed to bring Rust applications to ToaruOS. So far, Rust code can run on ToaruOS (as shown by from the Assembly syscall calling in src/main.rs).

Contributions are welcome.

You can test this by compiling ToaruOS (build instructions for a non-docker approach is in the works):

```sh
git clone https://github.com/klange/toaruos --depth 1
cd toaruos
git submodule update --depth 1 --init kuroko
docker pull toaruos/build-tools:1.99.x
docker run -v `pwd`:/root/misaka -w /root/misaka -e LANG=C.UTF-8 -t toaruos/build-tools:1.99.x util/build-in-docker.sh
```

and then by compiling toaru-rs with:

```sh
cargo build --release
```

After that you can copy the resulting toaru-rs binary (target/x86_64-toaru/release/toaru-rs) to base/usr/bin in the ToaruOS source tree and rerunning the docker command.

You can then run ToaruOS in the VM software of your choice and open the terminal and type "toaru-rs".

The long term goal of toaru-rs is to provide a target and base to link Rust code to ToaruOS components, allowing for the same integration you would have with ToaruOS in C. The point of toaru-rs is to allow for more language choices for developers experimenting with ToaruOS and not to rewrite parts of ToaruOS into Rust, keeping it in it's original C form.
