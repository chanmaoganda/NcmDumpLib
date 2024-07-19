# MusicDumpLib

This is a Rust library for dumping ncm files to mp3

binary example usage: [music_dumper](https://github.com/chanmaoganda/music_dumper)

basic usage:

```rust	
    let music_list: Vec<PathBuf> = get_items();
    let output_directory = PathBuf::from(output_path);
    let ncm_dumper = NcmDumper::new(music_list, output_directory);
    ncm_dumper.dump_all()?;
```

- ncm
ncm dump principle is [layout.md](src/ncm/layout.md)