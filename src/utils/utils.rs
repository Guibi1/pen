use semver::Version;

use crate::constants::{
    HOME_DIR, PEN_CONFIG_FILE, PEN_DIR, PYTHON_PACKAGES_DIR, PYTHON_VERSIONS_DIR, TMP_DIR,
};
use crate::utils::package::Package;
use std::{
    error::Error,
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
};

pub fn user_string_to_version(version: Option<&String>) -> Version {
    match version {
        Some(version) => {
            assert_major_minor_patch(version);
            match Version::parse(version) {
                Ok(version) => version,
                Err(e) => abort(&format!("Version {} is invalid.", version), Some(&e)),
            }
        }
        // TODO: Ask the user? Or maybe pick the most recent version?
        None => Version::parse("3.12.3").unwrap(),
    }
}

/// Asserts that a given version string adheres to the "major.minor.patch" format.
///
/// # Arguments
/// - `py_version`: A string slice representing the version number.
///
/// # Output
/// - None.
///
/// # Termination
/// - This function terminates if the `py_version` provided is not well formed.
///
/// # Guarantees
/// - If this function returns, it guarantees that `py_version` adheres to the format.
///
/// # Limitations
/// - The function does not validate if the given version is a valid Python version
pub fn assert_major_minor_patch(py_version: &str) {
    let parts = py_version.split('.').collect::<Vec<&str>>();

    if parts.len() != 3 {
        abort(&format!("Version {} does not match the major.minor.patch format : Version must have exactly three parts", py_version), None);
    }

    for part in parts {
        if part.parse::<u32>().is_err() {
            abort(&format!("Version {} does not match the major.minor.patch format : Each part must be a valid integer", py_version), None);
        }
    }
}

/// Constructs the path to the directory for a specified Python version without validating the format of the version string.
///
/// # Arguments
/// - `py_version`: A string slice representing the Python version, which has been
///   validated to conform to the expected format (major.minor.patch).
///
/// # Output
/// - A `PathBuf` pointing to the directory associated with the specified Python version.
///
/// # Termination
/// - This function does not terminate.
///
/// # Guarantees
/// - The returned path will be correctly formed if `py_version` is well formatted.
///
/// # Limitations
/// - The function does not validate the contents of the constructed path or its existence.
pub fn get_python_path(version: &Version) -> PathBuf {
    PYTHON_VERSIONS_DIR.join(format!(
        "{}.{}.{}",
        version.major, version.minor, version.patch
    ))
}

pub fn get_package_path(package: &Package) -> PathBuf {
    PYTHON_PACKAGES_DIR.join(format!(
        "{}_{}.{}.{}",
        package.name, package.version.major, package.version.minor, package.version.patch
    ))
}

/// Prompts the user to confirm an action.
///
/// # Arguments
/// - `prompt`: A string slice containing the prompt message to display to the user.
///
/// # Output
/// - Returns `true` if the user inputs "y" or "Y"; otherwise, returns `false`.
///
/// # Termination
/// - This function may terminate due to issues with input/output streams.
pub fn confirm_action(prompt: &str) -> bool {
    // todo should return result
    println!("{}", prompt);

    // Flush stdout to ensure the prompt appears before reading input
    if let Err(e) = io::stdout().flush() {
        abort("Failed to flush standart output", Some(&e));
    }

    // Read user input
    let mut user_input = String::new();
    if let Err(e) = io::stdin().read_line(&mut user_input) {
        abort("Failed to read standart input", Some(&e));
    }

    return user_input.trim().eq_ignore_ascii_case("y");
}

/// Downloads a file from a specified URL to a given file path. If a file already exists at the specified path, it will be deleted before the new file is downloaded
///
/// # Arguments
/// - `file_url`: A string slice representing the URL of the file to download.
/// - `file_path`: A `PathBuf` specifying where to save the downloaded file.
///
/// # Output
/// - None.
///
/// # Termination
/// - The function will terminate the process if it fails to remove an existing file.
/// - It will also terminate if an error occurs during the download.
/// - Additionally, it will terminate if the downloaded file cannot be found afterward.
///
/// # Guarantees
/// - The function guarantees the downloaded file exists.
///
/// # Limitations
/// - The function does not validate the contents of the downloaded file.
pub fn download_file(file_url: &str, file_path: &PathBuf) {
    // todo should return result
    if let Err(e) = fs::remove_file(file_path) {
        if e.kind() != io::ErrorKind::NotFound {
            abort("Unable to remove file", Some(&e));
        }
    }

    match process::Command::new("curl")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-4")
        .arg("-s")
        .arg("-o")
        .arg(file_path)
        .arg("-L")
        .arg(file_url)
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(
            &format!(
                "Failed to download file from {} to {}",
                file_url,
                file_path.display()
            ),
            None,
        ),
        Err(e) => abort(
            &format!(
                "Failed to extract Python version {} to {}",
                file_url,
                file_path.display()
            ),
            Some(&e),
        ),
    }

    if !file_path.exists() || !file_path.is_file() {
        abort("Downloaded file was not found", None);
    }
}

/// Attempts to delete a specified directory.
///
/// # Arguments
/// - `dir_path`: A `PathBuf` representing the directory to delete.
///
/// # Output
/// - Returns `Ok(())` if the directory was successfully deleted or if it was already empty.
/// - Returns an `Err` if the directory still exists after attempting deletion.
///
/// # Termination
/// - This function does not terminate.
///
/// # Guarantees
/// - If this function returns `Ok(())`, it guarantees that the directory no longer exists.
pub fn try_deleting_dir(dir_path: &PathBuf) -> Result<(), std::io::Error> {
    let delete_path = TMP_DIR.join("delete_path");
    return try_deleting_dir_to_temp(dir_path, &delete_path);
}

pub fn try_deleting_dir_to_temp(
    dir_path: &PathBuf,
    temp_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    if let Ok(exists) = dir_path.try_exists() {
        if !exists {
            return Ok(());
        }
    }
    match temp_dir.try_exists() {
        Ok(true) => fs::remove_dir_all(&temp_dir)?,
        Ok(false) => (),
        Err(e) => abort(
            &format!("Unable to know if {} exists", temp_dir.display()),
            Some(&e),
        ),
    }
    fs::rename(&dir_path, &temp_dir)?;
    if dir_path.try_exists()? {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Directory still exists",
        ))
    } else {
        Ok(())
    }
}

/// Checks if the specified dependencies are installed by running their `--help` command.
///
/// # Arguments
/// - `dependencies`: A vector of string slices representing the names of the dependencies to check.
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any dependencies, the `--help` command fails, this function terminates.
///
/// # Guarantees
/// - If the function returns, the dependencies are considered installed.
///
/// # Limitations
/// - The function only checks the result of the `--help` command for each dependencies.
pub fn assert_dependencies(dependencies: Vec<&'static str>) {
    for dep in dependencies {
        match process::Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", dep))
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()
        {
            Ok(status) if status.success() => continue,
            Ok(_) => abort(&format!("{} is not installed", dep), None),
            Err(e) => abort(
                &format!("Failed to check if {} is installed", dep),
                Some(&e),
            ),
        }
    }
}

/// Prints an error message and terminates the process.
///
/// # Input
/// - `message`: A string slice containing the error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Termination
/// - This function always terminates.
pub fn abort(message: &str, e: Option<&dyn Error>) -> ! {
    if let Some(error) = e {
        eprintln!("Error: {}: {}", message, error);
    } else {
        eprintln!("Error: {}", message);
    }
    process::exit(1);
}

/// Prints a critical error message and terminates the process with a status code of 1. The error message is prefixed with "Catastrophic failure: " and is highlighted in bold red text to emphasize the severity.
///
/// # Arguments
/// - `message`: A string slice containing the critical error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Termination
/// - This function always terminates.
pub fn catastrophic_failure(message: &str, e: Option<&dyn Error>) -> ! {
    const RED_BOLD: &str = "\x1b[1;31m"; // Bold red text
    const RESET: &str = "\x1b[0m"; // Reset formatting
    if let Some(error) = e {
        eprintln!(
            "{}Catastrophic failure: {}: {}{}",
            RED_BOLD, message, error, RESET
        );
    } else {
        eprintln!("{}Catastrophic failure: {}{}", RED_BOLD, message, RESET);
    }
    process::exit(1);
} // todo mabye possible to just print RED_BOLD then call abort idk

/// Clears and recreates the temporary directory.
///
/// # Input
/// - None.
///
/// # Output
/// - None.
///
/// # Termination
/// - If either removal or creation operations fail, the function prints an error message and terminates the process.
pub fn clear_temp() {
    let temp_is_empty = match (&*TMP_DIR).read_dir() {
        Ok(mut read_dir) => read_dir.next().is_none(),
        Err(e) => abort(
            &format!(
                "Failed to check contents of directory {}",
                (*TMP_DIR).display()
            ),
            Some(&e),
        ),
    };

    if temp_is_empty {
        return;
    }

    if let Err(e) = fs::remove_dir_all(&*TMP_DIR) {
        abort(
            &format!("Failed to clear directory {}", (*TMP_DIR).display()),
            Some(&e),
        )
    }

    if let Err(e) = fs::create_dir(&*TMP_DIR) {
        abort(
            &format!("Failed to create directory {}", (*TMP_DIR).display()),
            Some(&e),
        )
    }
}

/// Checks if the paths used in pen exists.
///
/// # Arguments
/// - None
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any paths, checking the existence fails or returns false, this function exits. Only exeption is if `PYTHON_VERSIONS_DIR` does not exist, then it it created.
///
/// # Guarantees
/// - If the function returns, the paths are considered to be existing.
pub fn assert_global_paths() {
    match HOME_DIR.try_exists() {
        Ok(true) => {
            // not the same as .is_file() because of error coercion
            if !HOME_DIR.is_dir() {
                abort("todo", None);
            }
        }
        Ok(false) => abort(&format!("{} does not exist.", HOME_DIR.display()), None),
        Err(e) => abort(
            &format!("Failed to check if directory {} exists", HOME_DIR.display()),
            Some(&e),
        ),
    }

    // No need to check for PEN_BIN since it is only used when uninstalling, where it is checked for existence.
    let dirs_to_check = vec![
        (&*PEN_DIR),
        (&*PYTHON_PACKAGES_DIR),
        (&*TMP_DIR),
        (&*PYTHON_VERSIONS_DIR),
    ];

    for path in dirs_to_check {
        match path.try_exists() {
            Ok(true) => {
                if path.is_dir() {
                    continue;
                } else {
                    abort("todo", None);
                }
            }
            Ok(false) => {
                println!("{}", path.display());
                if let Err(e) = fs::create_dir_all(path) {
                    abort(
                        &format!("Failed to create directory {}", path.display()),
                        Some(&e),
                    );
                }
            }
            Err(e) => abort(
                &format!("Failed to check if directory {} exists", path.display()),
                Some(&e),
            ),
        }
    }

    match PEN_CONFIG_FILE.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            if let Err(e) = fs::File::create_new(&*PEN_CONFIG_FILE) {
                abort("todo", Some(&e));
            }
        }
        Err(e) => abort(
            &format!("Failed to check if file {} exists", PEN_DIR.display()),
            Some(&e),
        ),
    }
}
