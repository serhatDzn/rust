### Derleme
```
rustc hello_rust.rs
```

### Kararsız Sürüm
```
rustup override set nightly
```

### -Z Çıktısı
```
rustc -Z -h
rustc -Z unpretty=normal hello_rust.rs 
rustc -Z unpretty=expanded hello_rust.rs 
rustc -Z unpretty=hir,typed hello_rust.rs 
rustc -Z unpretty=mir hello_rust.rs 
```

### Obj Çıktısı
```
rustc --emit llvm-ir hello_rust.rs
rustc --emit obj hello_rust.rs
```