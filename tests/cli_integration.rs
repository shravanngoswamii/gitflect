use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn tool() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_gitflect"))
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn scratch_dir(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let dir = manifest_dir()
        .join("target")
        .join("integration-tests")
        .join(format!("{name}-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&dir).expect("failed to create integration test scratch directory");
    dir
}

fn write(path: impl AsRef<Path>, contents: &str) {
    fs::write(path, contents).expect("failed to write test fixture");
}

fn run(mut command: Command) -> Output {
    let output = command.output().expect("failed to spawn test command");
    if !output.status.success() {
        panic!(
            "command failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    output
}

fn run_tool(cwd: &Path, args: &[&str]) -> String {
    run_tool_with_env(cwd, args, &[])
}

fn run_tool_with_env(cwd: &Path, args: &[&str], envs: &[(&str, &str)]) -> String {
    let home = cwd.join(".home");
    let xdg_config = cwd.join(".config");
    fs::create_dir_all(&home).expect("failed to create isolated HOME");
    fs::create_dir_all(&xdg_config).expect("failed to create isolated XDG_CONFIG_HOME");
    let git_ceiling = cwd.parent().unwrap_or(cwd);

    let mut command = Command::new(tool());
    command
        .args(args)
        .current_dir(cwd)
        .env("GIT_CEILING_DIRECTORIES", git_ceiling)
        .env("HOME", &home)
        .env("XDG_CONFIG_HOME", &xdg_config)
        .env("GITFLECT_CONFIG", cwd.join("missing-config"))
        .env("GITFLECT_COLOR", "never")
        .env("GITFLECT_SHOW_ZERO", "true")
        .env("GITFLECT_UNTRACKED_FILES", "normal")
        .env("GITFLECT_ENABLE_STASH_STATUS", "false")
        .env("GITFLECT_ABBREVIATE_GIT_DIR", "false")
        .env("GITFLECT_ABBREVIATE_HOME", "false")
        .env("NO_COLOR", "1");

    for (key, value) in envs {
        command.env(key, value);
    }

    String::from_utf8(run(command).stdout).expect("tool output should be utf-8")
}

fn git<I, S>(cwd: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command
        .args(args)
        .current_dir(cwd)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Integration Test")
        .env("GIT_AUTHOR_EMAIL", "integration@example.invalid")
        .env("GIT_COMMITTER_NAME", "Integration Test")
        .env("GIT_COMMITTER_EMAIL", "integration@example.invalid");
    run(command);
}

fn init_repo(root: &Path) -> PathBuf {
    let repo = root.join("repo");
    fs::create_dir_all(&repo).expect("failed to create test repository");
    git(&repo, ["-c", "init.defaultBranch=main", "init"]);
    git(&repo, ["config", "user.name", "Integration Test"]);
    git(
        &repo,
        ["config", "user.email", "integration@example.invalid"],
    );
    repo
}

fn commit_all(repo: &Path, message: &str) {
    git(repo, ["add", "."]);
    git(repo, ["commit", "-m", message]);
}

fn current_target() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-gnu"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        _ => None,
    }
}

#[test]
fn status_is_empty_outside_git_repository() {
    let root = scratch_dir("outside-repo");

    let output = run_tool(&root, ["status", "--no-color"].as_slice());

    assert_eq!(output, "");
}

#[test]
fn status_and_json_report_real_git_counts() {
    let root = scratch_dir("dirty-status");
    let repo = init_repo(&root);
    write(repo.join("tracked.txt"), "one\n");
    write(repo.join("removed.txt"), "remove me\n");
    commit_all(&repo, "initial");

    write(repo.join("tracked.txt"), "one\ntwo\n");
    fs::remove_file(repo.join("removed.txt")).expect("failed to delete tracked fixture");
    write(repo.join("staged.txt"), "new staged file\n");
    git(&repo, ["add", "staged.txt"]);
    write(repo.join("untracked.txt"), "new working file\n");

    let status = run_tool(&repo, ["status", "--no-color"].as_slice());
    assert_eq!(status.trim(), "(main +1 ~0 -0 | +1 ~1 -1 !)");

    let json = run_tool(&repo, ["status", "--json", "--no-color"].as_slice());
    assert!(json.contains("\"branch\":\"main\""));
    assert!(json.contains("\"has_index\":true"));
    assert!(json.contains("\"has_working\":true"));
    assert!(json.contains("\"has_untracked\":true"));
    assert!(json.contains("\"index\":{\"added\":[\"staged.txt\"]"));
    assert!(json.contains("\"working\":{\"added\":[\"untracked.txt\"]"));
    assert!(json.contains("\"modified\":[\"tracked.txt\"]"));
    assert!(json.contains("\"deleted\":[\"removed.txt\"]"));
}

#[test]
fn upstream_divergence_is_rendered_after_fetch() {
    let root = scratch_dir("upstream-divergence");
    let remote = root.join("remote.git");
    fs::create_dir_all(&remote).expect("failed to create bare remote directory");
    git(&remote, ["-c", "init.defaultBranch=main", "init", "--bare"]);

    let local = root.join("local");
    fs::create_dir_all(&local).expect("failed to create local clone directory");
    git(&local, ["-c", "init.defaultBranch=main", "init"]);
    git(&local, ["config", "user.name", "Integration Test"]);
    git(
        &local,
        ["config", "user.email", "integration@example.invalid"],
    );
    git(
        &local,
        ["remote", "add", "origin", remote.to_str().unwrap()],
    );
    write(local.join("shared.txt"), "initial\n");
    commit_all(&local, "initial");
    git(&local, ["push", "-u", "origin", "main"]);

    let other = root.join("other");
    git(
        &root,
        ["clone", remote.to_str().unwrap(), other.to_str().unwrap()],
    );
    git(&other, ["config", "user.name", "Integration Test"]);
    git(
        &other,
        ["config", "user.email", "integration@example.invalid"],
    );
    write(other.join("remote-only.txt"), "remote\n");
    commit_all(&other, "remote commit");
    git(&other, ["push", "origin", "main"]);

    write(local.join("local-only.txt"), "local\n");
    commit_all(&local, "local commit");
    git(&local, ["fetch", "origin"]);

    let status = run_tool(&local, ["status", "--no-color"].as_slice());
    assert_eq!(status.trim(), "(main ↓1 ↑1)");
}

#[test]
fn stash_count_is_opt_in_and_config_file_driven() {
    let root = scratch_dir("stash-config");
    let repo = init_repo(&root);
    write(repo.join("tracked.txt"), "one\n");
    commit_all(&repo, "initial");
    write(repo.join("tracked.txt"), "one\ntwo\n");
    git(&repo, ["stash", "push", "-m", "saved work"]);

    let config = root.join("prompt.conf");
    write(
        &config,
        concat!(
            "color=never\n",
            "enable_stash_status=true\n",
            "show_zero_counts=true\n",
        ),
    );

    let output = run_tool_with_env(
        &repo,
        ["status", "--no-color"].as_slice(),
        &[
            ("GITFLECT_CONFIG", config.to_str().unwrap()),
            ("GITFLECT_ENABLE_STASH_STATUS", "true"),
        ],
    );

    assert_eq!(output.trim(), "(main (1))");
}

#[test]
fn merge_state_is_visible_during_conflicts() {
    let root = scratch_dir("merge-state");
    let repo = init_repo(&root);
    write(repo.join("conflict.txt"), "base\n");
    commit_all(&repo, "initial");

    git(&repo, ["checkout", "-b", "feature"]);
    write(repo.join("conflict.txt"), "feature\n");
    commit_all(&repo, "feature edit");

    git(&repo, ["checkout", "main"]);
    write(repo.join("conflict.txt"), "main\n");
    commit_all(&repo, "main edit");

    let output = Command::new("git")
        .args(["merge", "feature"])
        .current_dir(&repo)
        .output()
        .expect("failed to run conflicting merge");
    assert!(
        !output.status.success(),
        "merge should conflict so operation state can be tested"
    );

    let status = run_tool(&repo, ["status", "--no-color"].as_slice());
    assert!(status.contains("main|MERGING"), "{status}");
    assert!(status.contains("!1"), "{status}");
}

#[test]
fn generated_shell_init_uses_status_only_segments() {
    let root = scratch_dir("init-output");

    let bash = run_tool(&root, ["init", "bash"].as_slice());
    assert!(bash.contains("# gitflect Bash integration"));
    assert!(bash.contains("prompt --shell bash --status-only"));
    assert!(bash.contains("__gitflect_apply_segment"));

    let zsh = run_tool(&root, ["init", "zsh"].as_slice());
    assert!(zsh.contains("# gitflect Zsh integration"));
    assert!(zsh.contains("prompt --shell zsh --status-only"));
    assert!(zsh.contains("add-zsh-hook precmd __gitflect_precmd"));
}

#[test]
fn installer_can_install_from_a_local_release_archive() {
    let Some(target) = current_target() else {
        return;
    };

    let root = scratch_dir("installer");
    let release_dir = root.join("release");
    let archive_name = format!("gitflect-{target}");
    let archive_dir = release_dir.join(&archive_name);
    let bin_dir = root.join("bin");
    let profile = root.join("bashrc");
    fs::create_dir_all(&archive_dir).expect("failed to create local archive directory");
    fs::copy(tool(), archive_dir.join("gitflect"))
        .expect("failed to copy test binary into local archive");
    write(archive_dir.join("README.md"), "test archive\n");
    write(archive_dir.join("LICENSE"), "test license\n");

    let mut tar = Command::new("tar");
    tar.args(["-czf", &format!("{archive_name}.tar.gz"), &archive_name])
        .current_dir(&release_dir);
    run(tar);

    let mut install = Command::new("sh");
    install
        .arg("install.sh")
        .arg("--base-url")
        .arg(format!("file://{}", release_dir.display()))
        .arg("--bin-dir")
        .arg(&bin_dir)
        .arg("--shell")
        .arg("bash")
        .arg("--profile")
        .arg(&profile)
        .current_dir(manifest_dir());
    run(install);

    let installed = bin_dir.join("gitflect");
    assert!(installed.is_file(), "installer should write the binary");

    let mut version = Command::new(&installed);
    version.arg("--version").current_dir(&root);
    let version_output = String::from_utf8(run(version).stdout).expect("version should be utf-8");
    assert!(version_output.starts_with("gitflect "));

    let profile_contents = fs::read_to_string(profile).expect("installer should update profile");
    assert!(profile_contents.contains("# >>> gitflect >>>"));
    assert!(profile_contents.contains(&format!("export PATH=\"{}:$PATH\"", bin_dir.display())));
    assert!(profile_contents.contains("eval \"$(gitflect init bash)\""));
}
