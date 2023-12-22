### Proje oluşturma
```
cargo new primitives --vcs=none
```

### Değişken isimlendirme
```
* harf veya _ ile başlar
* aralarında boşluk bırakılmaz
* özel karakterler kullanılmaz
* sayı ile başlatılamaz
* rust snake case isimlendirme tavsiye eder
like: age_of_empires
* 
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