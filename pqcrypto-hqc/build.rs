extern crate cc;
extern crate glob;

use std::path::Path;

fn main() {
    let common_dir = Path::new("pqclean/common");
    let common_files = vec![
        common_dir.join("nistseedexpander.c"),
        common_dir.join("fips202.c"),
        common_dir.join("aes.c"),
        common_dir.join("sha2.c"),
        common_dir.join("randombytes.c"),
        common_dir.join("sp800-185.c"),
    ];

    cc::Build::new()
        .flag("-std=c99")
        .include("pqclean/common")
        .files(common_files.into_iter())
        .compile("pqclean_common");

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-128-1-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-128-1-cca2_leaktime");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-192-1-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-192-1-cca2_leaktime");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-192-2-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-192-2-cca2_leaktime");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-256-1-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-256-1-cca2_leaktime");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-256-2-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-256-2-cca2_leaktime");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_kem/hqc-256-3-cca2/leaktime");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("hqc-256-3-cca2_leaktime");
    }
}
