use just_core::result::BoxedResult;
use std::path::Path;

const _7Z_EXT: [&str; 12] = [
    "gz", "tar", "tgz", "lzma", "bz", "bz2", "7z", "rar", "iso", "xz", "lzh", "nupkg",
];

enum Extension {
    Zip,
    Msi,
    _7z,
    Unsupported(String),
    Unknown,
}

impl Extension {
    fn from_path(path: &Path) -> Self {
        if let Some(ext) = path.extension() {
            if let Some(ext) = ext.to_str() {
                Self::from_extension(ext)
            } else {
                Extension::Unsupported(ext.to_string_lossy().into())
            }
        } else {
            Extension::Unknown
        }
    }

    fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "zip" => Extension::Zip,
            "msi" => Extension::Msi,
            _ => {
                let extension = ext.to_owned();
                if _7Z_EXT.contains(&ext) {
                    Extension::_7z
                } else {
                    Extension::Unsupported(extension)
                }
            }
        }
    }
}

pub fn extract(uncompressed_path: &Path, compressed_path: &Path) -> BoxedResult<()> {
    use log::info;

    info!(
        "Extracting {:?} into {:?}",
        compressed_path, uncompressed_path
    );

    match Extension::from_path(uncompressed_path) {
        Extension::Zip => extract_zip(uncompressed_path, compressed_path),
        Extension::Msi => extract_msi(uncompressed_path, compressed_path),
        Extension::_7z => extract_7z(uncompressed_path, compressed_path),
        Extension::Unsupported(ext) => panic!("Unsupported extension {}", ext),
        Extension::Unknown => panic!("No or unknown extension"),
    }
}

fn extract_zip(uncompressed_path: &Path, compressed_path: &Path) -> BoxedResult<()> {
    use just_core::system::cmd_run;
    use log::debug;

    debug!("Extracting with zip");

    cmd_run("unzip", &[uncompressed_path, compressed_path])
}

fn extract_7z(uncompressed_path: &Path, compressed_path: &Path) -> BoxedResult<()> {
    use just_core::system::cmd_run;
    use log::debug;

    debug!("Extracting with 7z");

    let compressed_filename = compressed_path.to_string_lossy();
    let uncompressed_filename = uncompressed_path.to_string_lossy();
    let output = format!("-o{}", uncompressed_filename);

    cmd_run("7z", &["x", &compressed_filename, &output, "-y"])
}

fn extract_msi(uncompressed_path: &Path, compressed_path: &Path) -> BoxedResult<()> {
    use just_core::system::cmd_run;
    use log::debug;

    debug!("Extracting with msi");

    let compressed_filename = compressed_path.to_string_lossy();
    let uncompressed_filename = uncompressed_path.to_string_lossy();

    let target = format!("TARGETDIR=\"{}\"", uncompressed_filename);

    cmd_run(
        "msiexec",
        &["/a", &compressed_filename, "/qn", &target, "/lwe", "log"],
    )
}
