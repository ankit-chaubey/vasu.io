use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Ankit Chaubey";
const GITHUB: &str = "github.com/ankit-chaubey";

// ─────────────────────────────────────────────────────────────
//  CLI STRUCTURE
// ─────────────────────────────────────────────────────────────
#[derive(Parser)]
#[command(
    name    = "vasu",
    about   = "Ankit Chaubey's personal power toolkit 🛠️",
    version = VERSION,
    author  = AUTHOR,
    arg_required_else_help = false,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete everything in CWD except listed items  [e.g. vasu del .git vasu]
    Del {
        /// Items to KEEP (everything else gets deleted)
        #[arg(required = true, num_args = 1..)]
        keep: Vec<String>,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Deep-copy SRC → DST (hidden files, permissions, symlinks)
    Cp {
        /// Source file or folder
        src: PathBuf,
        /// Destination path
        dst: PathBuf,
        /// Overwrite destination if it exists
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Copy file contents to clipboard
    Cb {
        /// Files, globs, or dirs to copy. Use * for everything recursively.
        #[arg(num_args = 0..)]
        targets: Vec<String>,
        /// No file-name headers between files
        #[arg(long)]
        no_header: bool,
    },

    /// Pretty directory tree
    Tree {
        /// Root directory (default: current)
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Max depth
        #[arg(short, long, default_value = "4")]
        depth: usize,
        /// Show hidden files
        #[arg(short, long)]
        all: bool,
    },

    /// Find files by name pattern (supports * wildcards)
    Find {
        /// Pattern to match (e.g. "*.rs")
        pattern: String,
        /// Root directory
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Filter: f=files, d=dirs
        #[arg(short = 't', long, default_value = "all",
              value_parser = ["f","d","all"])]
        ftype: String,
    },

    /// Disk usage per item, sorted by size
    Size {
        /// Root directory
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Show top N items
        #[arg(short = 'n', long, default_value = "20")]
        top: usize,
    },

    /// Remove build artifacts and junk files
    Clean {
        /// Root directory
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// Zip a file or folder
    Zip {
        /// Source to zip
        source: PathBuf,
        /// Output zip file (optional)
        output: Option<PathBuf>,
    },

    /// Unzip an archive
    Unzip {
        /// Archive file
        archive: PathBuf,
        /// Destination folder
        #[arg(default_value = ".")]
        destination: PathBuf,
    },

    /// Bulk rename files: replace PATTERN with REPLACEMENT
    Rename {
        /// String to find in filenames
        pattern: String,
        /// String to replace with
        replacement: String,
        /// Directory to scan
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Preview only, don't rename
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// Count files and total lines of code
    Count {
        /// Root directory
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Filter by extension (e.g. -e rs -e toml)
        #[arg(short, long)]
        ext: Vec<String>,
    },

    /// Show MD5 and SHA256 hash of a file
    Hash {
        /// File to hash
        file: PathBuf,
    },

    /// Create a timestamped zip backup
    Backup {
        /// Source to back up
        #[arg(default_value = ".")]
        source: PathBuf,
        /// Where to place the backup
        #[arg(short, long, default_value = ".")]
        dest: PathBuf,
    },

    /// Print environment variables (optionally filtered)
    Env {
        /// Substring filter
        #[arg(default_value = "")]
        filter: String,
    },

    /// Spin up a quick HTTP file server
    Http {
        /// Port number
        #[arg(default_value = "8080")]
        port: u16,
        /// Directory to serve
        #[arg(default_value = ".")]
        directory: PathBuf,
    },

    /// Compare two directories
    Diff {
        /// First directory
        dir_a: PathBuf,
        /// Second directory
        dir_b: PathBuf,
    },

    /// Find duplicate files by content hash
    Dupe {
        /// Root directory
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
}

// ─────────────────────────────────────────────────────────────
//  MAIN
// ─────────────────────────────────────────────────────────────
fn main() {
    let cli = Cli::parse();
    match cli.command {
        None                   => show_banner(),
        Some(Commands::Del   { keep, yes })              => cmd_del(keep, yes),
        Some(Commands::Cp    { src, dst, overwrite })    => cmd_cp(&src, &dst, overwrite),
        Some(Commands::Cb    { targets, no_header })     => cmd_cb(targets, no_header),
        Some(Commands::Tree  { directory, depth, all })  => cmd_tree(&directory, depth, all),
        Some(Commands::Find  { pattern, directory, ftype }) => cmd_find(&pattern, &directory, &ftype),
        Some(Commands::Size  { directory, top })         => cmd_size(&directory, top),
        Some(Commands::Clean { directory, yes })         => cmd_clean(&directory, yes),
        Some(Commands::Zip   { source, output })         => cmd_zip(&source, output),
        Some(Commands::Unzip { archive, destination })   => cmd_unzip(&archive, &destination),
        Some(Commands::Rename { pattern, replacement, directory, dry_run }) =>
            cmd_rename(&pattern, &replacement, &directory, dry_run),
        Some(Commands::Count { directory, ext })         => cmd_count(&directory, ext),
        Some(Commands::Hash  { file })                   => cmd_hash(&file),
        Some(Commands::Backup { source, dest })          => cmd_backup(&source, &dest),
        Some(Commands::Env   { filter })                 => cmd_env(&filter),
        Some(Commands::Http  { port, directory })        => cmd_http(port, &directory),
        Some(Commands::Diff  { dir_a, dir_b })           => cmd_diff(&dir_a, &dir_b),
        Some(Commands::Dupe  { directory })              => cmd_dupe(&directory),
    }
}

// ─────────────────────────────────────────────────────────────
//  BANNER
// ─────────────────────────────────────────────────────────────
fn show_banner() {
    println!("{}", r#"
  ██╗   ██╗ █████╗ ███████╗██╗   ██╗
  ██║   ██║██╔══██╗██╔════╝██║   ██║
  ██║   ██║███████║███████╗██║   ██║
  ╚██╗ ██╔╝██╔══██║╚════██║██║   ██║
   ╚████╔╝ ██║  ██║███████║╚██████╔╝
    ╚═══╝  ╚═╝  ╚═╝╚══════╝ ╚═════╝"#.cyan().bold());

    println!("  {}  ·  {}  ·  {}\n",
        format!("v{VERSION}").green().bold(),
        AUTHOR.yellow(),
        GITHUB.bright_blue().underline(),
    );

    let cmds: &[(&str, &str)] = &[
        ("vasu",               "Show this banner"),
        ("vasu del .git vasu", "Delete everything EXCEPT listed items"),
        ("vasu cp src/ dst/",  "Deep-copy (hidden files, perms, symlinks)"),
        ("vasu cb *",          "Copy ALL file contents to clipboard (recursive)"),
        ("vasu cb f1 f2 dir/", "Copy specific files/globs to clipboard"),
        ("vasu tree",          "Pretty directory tree"),
        ("vasu find '*.rs'",   "Find files by name pattern"),
        ("vasu size",          "Disk usage per item, sorted"),
        ("vasu clean",         "Remove build artifacts & junk"),
        ("vasu zip src/",      "Zip a file/folder"),
        ("vasu unzip f.zip",   "Unzip an archive"),
        ("vasu rename p r",    "Bulk rename files"),
        ("vasu count",         "Count files & lines of code"),
        ("vasu hash file",     "Show MD5/SHA256"),
        ("vasu backup",        "Timestamped zip backup"),
        ("vasu env [filter]",  "Print env vars"),
        ("vasu http [port]",   "Quick HTTP file server"),
        ("vasu diff a/ b/",    "Compare two directories"),
        ("vasu dupe",          "Find duplicate files"),
    ];

    println!("  {:<32} {}", "COMMAND".bold().underline(), "DESCRIPTION".bold().underline());
    for (cmd, desc) in cmds {
        println!("  {:<32} {}", cmd.yellow(), desc.white());
    }
    println!("\n  Run {} for details\n", "vasu <command> --help".cyan());
}

// ─────────────────────────────────────────────────────────────
//  DEL
// ─────────────────────────────────────────────────────────────
fn cmd_del(keep: Vec<String>, yes: bool) {
    let cwd = std::env::current_dir().unwrap();
    let keep_set: std::collections::HashSet<_> = keep.iter().collect();

    let to_delete: Vec<_> = std::fs::read_dir(&cwd)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| !keep_set.contains(&e.file_name().to_string_lossy().to_string()))
        .collect();

    if to_delete.is_empty() {
        println!("{}", "Nothing to delete — everything is already kept.".green());
        return;
    }

    println!("\n{} {} item(s) in {}:",
        "Will DELETE".red().bold(),
        to_delete.len(),
        cwd.display().to_string().yellow(),
    );
    for item in &to_delete {
        let name = item.file_name().to_string_lossy().to_string();
        let icon = if item.path().is_dir() { "📁" } else { "📄" };
        println!("  {icon}  {}", name.red());
    }
    println!("\n{} {}", "Will KEEP:".green().bold(), keep.join(", ").cyan());

    if !yes && !confirm("\nProceed?") { return; }

    let mut count = 0usize;
    for item in &to_delete {
        let p = item.path();
        let result = if p.is_dir() {
            std::fs::remove_dir_all(&p).map_err(|e| e.to_string())
        } else {
            std::fs::remove_file(&p).map_err(|e| e.to_string())
        };
        match result {
            Ok(_) => count += 1,
            Err(e) => eprintln!("  {} {}: {}", "✗".red(), p.display(), e),
        }
    }
    println!("\n{} Deleted {} item(s).", "✓".green().bold(), count);
}

// ─────────────────────────────────────────────────────────────
//  CP
// ─────────────────────────────────────────────────────────────
fn cmd_cp(src: &Path, dst: &Path, overwrite: bool) {
    if !src.exists() {
        eprintln!("{} Source not found: {}", "✗".red(), src.display());
        std::process::exit(1);
    }
    if dst.exists() && !overwrite {
        eprintln!("{} Destination exists. Use --overwrite / -o to force.", "!".yellow());
        std::process::exit(1);
    }

    let opts = fs_extra::dir::CopyOptions {
        overwrite,
        copy_inside: true,
        content_only: false,
        ..Default::default()
    };

    print!("Copying {} → {} … ", src.display().to_string().cyan(), dst.display().to_string().cyan());

    if src.is_dir() {
        if let Err(e) = fs_extra::dir::copy(src, dst, &opts) {
            eprintln!("{} {}", "✗".red(), e);
            std::process::exit(1);
        }
    } else {
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        if let Err(e) = std::fs::copy(src, dst) {
            eprintln!("{} {}", "✗".red(), e);
            std::process::exit(1);
        }
    }
    println!("{}", "done!".green().bold());
}

// ─────────────────────────────────────────────────────────────
//  CB  — clipboard
// ─────────────────────────────────────────────────────────────
fn cmd_cb(targets: Vec<String>, no_header: bool) {
    let effective: Vec<String> = if targets.is_empty() {
        vec![".".to_string()]
    } else {
        targets
    };

    let mut files: Vec<PathBuf> = Vec::new();

    for t in &effective {
        let p = PathBuf::from(t);
        if p.is_dir() {
            collect_files_recursive(&p, &mut files);
        } else if p.is_file() {
            files.push(p);
        } else {
            for entry in glob::glob(t).into_iter().flatten().flatten() {
                if entry.is_file() {
                    files.push(entry);
                } else if entry.is_dir() {
                    collect_files_recursive(&entry, &mut files);
                }
            }
        }
    }

    if files.is_empty() {
        println!("{}", "No files found.".yellow());
        return;
    }

    let mut parts: Vec<String> = Vec::new();
    let mut count = 0usize;

    for fp in &files {
        if let Ok(content) = std::fs::read_to_string(fp) {
                if no_header {
                    parts.push(content);
                } else {
                    parts.push(format!("\n\n# ─── {} ───\n\n{}", fp.display(), content));
                }
                count += 1;
        }
    }

    let full = parts.join("\n");
    let size_kb = full.len() as f64 / 1024.0;

    match copy_to_clipboard(&full) {
        Ok(method) => {
            println!("{} Copied {} file(s) ({:.1} KB) to clipboard  [via {}]",
                "✓".green().bold(), count, size_kb, method.dimmed());
            for fp in files.iter().take(8) {
                println!("    {}", fp.display().to_string().cyan().dimmed());
            }
            if files.len() > 8 {
                println!("    … and {} more", files.len() - 8);
            }
        }
        Err(_) => {
            // Last resort: dump to stdout so the user can pipe it
            eprintln!("{} No clipboard tool found. Printing to stdout (pipe it yourself).", "!".yellow());
            println!("{}", full);
        }
    }
}

/// Try clipboard tools in order of preference:
///   1. termux-clipboard-set  (Android / Termux)
///   2. pbcopy                (macOS)
///   3. xclip                 (Linux + X11)
///   4. xsel                  (Linux + X11 fallback)
///   5. wl-copy               (Wayland)
fn copy_to_clipboard(text: &str) -> Result<&'static str, ()> {
    let tools: &[(&[&str], &str)] = &[
        (&["termux-clipboard-set"],          "termux-clipboard-set"),
        (&["pbcopy"],                         "pbcopy"),
        (&["xclip", "-selection", "clipboard"], "xclip"),
        (&["xsel", "--clipboard", "--input"], "xsel"),
        (&["wl-copy"],                        "wl-copy"),
    ];

    for (args, label) in tools {
        let mut cmd = std::process::Command::new(args[0]);
        for arg in &args[1..] { cmd.arg(arg); }
        cmd.stdin(std::process::Stdio::piped())
           .stdout(std::process::Stdio::null())
           .stderr(std::process::Stdio::null());

        if let Ok(mut child) = cmd.spawn() {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(text.as_bytes()).ok();
            }
            if child.wait().map(|s| s.success()).unwrap_or(false) {
                return Ok(label);
            }
        }
    }
    Err(())
}

fn collect_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    use walkdir::WalkDir;
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            out.push(entry.into_path());
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  TREE
// ─────────────────────────────────────────────────────────────
fn cmd_tree(directory: &Path, depth: usize, show_all: bool) {
    println!("{}", directory.canonicalize().unwrap_or(directory.to_path_buf()).display()
        .to_string().cyan().bold());
    print_tree(directory, "", depth, 0, show_all);
}

fn print_tree(dir: &Path, prefix: &str, max_depth: usize, current: usize, show_all: bool) {
    if current >= max_depth { return; }

    let mut entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };

    if !show_all {
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
    }

    entries.sort_by_key(|e| {
        let is_file = e.path().is_file();
        (is_file, e.file_name().to_string_lossy().to_lowercase().to_string())
    });

    let count = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i + 1 == count;
        let connector = if is_last { "└── " } else { "├── " };
        let extension = if is_last { "    " } else { "│   " };
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if path.is_dir() {
            println!("{}{}{}", prefix, connector, name.blue().bold());
            print_tree(&path, &format!("{prefix}{extension}"), max_depth, current + 1, show_all);
        } else {
            let size = path.metadata().map(|m| human_size(m.len())).unwrap_or_default();
            println!("{}{}{} {}",
                prefix, connector,
                name.green(),
                size.dimmed(),
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  FIND
// ─────────────────────────────────────────────────────────────
fn cmd_find(pattern: &str, directory: &Path, ftype: &str) {
    use walkdir::WalkDir;
    let mut results: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        if !glob_match(pattern, &name) { continue; }
        match ftype {
            "f" if !entry.file_type().is_file() => continue,
            "d" if !entry.file_type().is_dir()  => continue,
            _ => {}
        }
        results.push(entry.into_path());
    }

    if results.is_empty() {
        println!("{}", format!("No matches for '{pattern}'").yellow());
        return;
    }

    println!("\n{}\n", format!("Found {} match(es):", results.len()).green().bold());
    let base = directory.canonicalize().unwrap_or(directory.to_path_buf());
    for r in &results {
        let rel = r.strip_prefix(&base).unwrap_or(r);
        let icon = if r.is_dir() { "📁" } else { "📄" };
        println!("  {icon}  {}", rel.display().to_string().cyan());
    }
    println!();
}

// Simple glob: supports * wildcard
fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" { return true; }
    let lower_p = pattern.to_lowercase();
    let lower_t = text.to_lowercase();
    if !lower_p.contains('*') {
        return lower_t.contains(&lower_p);
    }
    let parts: Vec<&str> = lower_p.split('*').collect();
    let mut pos = 0usize;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() { continue; }
        if i == 0 {
            if !lower_t.starts_with(part) { return false; }
            pos = part.len();
        } else {
            match lower_t[pos..].find(part) {
                Some(p) => pos += p + part.len(),
                None    => return false,
            }
        }
    }
    if lower_p.ends_with('*') { true } else { pos == lower_t.len() || lower_t.ends_with(parts.last().unwrap_or(&"")) }
}

// ─────────────────────────────────────────────────────────────
//  SIZE
// ─────────────────────────────────────────────────────────────
fn cmd_size(directory: &Path, top: usize) {
    let mut entries: Vec<(u64, PathBuf)> = std::fs::read_dir(directory)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| {
            let p = e.path();
            let size = dir_size(&p);
            (size, p)
        })
        .collect();

    entries.sort_by(|a, b| b.0.cmp(&a.0));

    println!("\n  {:<12} {}", "SIZE".bold().underline(), "ITEM".bold().underline());
    for (size, path) in entries.iter().take(top) {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let icon = if path.is_dir() { "📁" } else { "📄" };
        println!("  {:<12} {icon}  {}", human_size(*size).yellow(), name.white());
    }
    println!();
}

fn dir_size(path: &Path) -> u64 {
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

// ─────────────────────────────────────────────────────────────
//  CLEAN
// ─────────────────────────────────────────────────────────────
const CLEAN_NAMES: &[&str] = &[
    "__pycache__", ".pytest_cache", ".mypy_cache", ".ruff_cache",
    "target", ".DS_Store", "Thumbs.db", ".eggs",
];
const CLEAN_EXTS: &[&str] = &[".pyc", ".pyo", ".class", ".o", ".obj", ".log"];

fn cmd_clean(directory: &Path, yes: bool) {
    use walkdir::WalkDir;
    let mut found: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_junk = CLEAN_NAMES.contains(&name.as_str())
            || CLEAN_EXTS.iter().any(|ext| name.ends_with(ext));
        if is_junk { found.push(entry.into_path()); }
    }

    // dedupe: remove paths that are children of already-found dirs
    found.dedup_by(|a, b| a.starts_with(b.as_path()));

    if found.is_empty() {
        println!("{}", "Nothing to clean — already spotless! ✨".green());
        return;
    }

    println!("\n{} {} item(s):", "Will remove".yellow().bold(), found.len());
    let base = directory.canonicalize().unwrap_or(directory.to_path_buf());
    for f in found.iter().take(15) {
        let rel = f.strip_prefix(&base).unwrap_or(f);
        println!("  {}", rel.display().to_string().red().dimmed());
    }
    if found.len() > 15 { println!("  … and {} more", found.len() - 15); }

    if !yes && !confirm("\nProceed?") { return; }

    let mut removed = 0usize;
    for f in &found {
        let ok = if f.is_dir() {
            std::fs::remove_dir_all(f).is_ok()
        } else {
            std::fs::remove_file(f).is_ok()
        };
        if ok { removed += 1; }
    }
    println!("{} Cleaned {} item(s).", "✓".green().bold(), removed);
}

// ─────────────────────────────────────────────────────────────
//  ZIP
// ─────────────────────────────────────────────────────────────
fn cmd_zip(source: &Path, output: Option<PathBuf>) {
    use std::io::Write;

    let out = output.unwrap_or_else(|| {
        PathBuf::from(format!("{}.zip", source.file_name().unwrap().to_string_lossy()))
    });

    let file = std::fs::File::create(&out).expect("Cannot create zip file");
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut count = 0usize;
    if source.is_dir() {
        for entry in walkdir::WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let rel = path.strip_prefix(source.parent().unwrap_or(Path::new("."))).unwrap();
            if path.is_file() {
                zip.start_file(rel.to_string_lossy(), opts).unwrap();
                let data = std::fs::read(path).unwrap();
                zip.write_all(&data).unwrap();
                count += 1;
            } else if path.is_dir() {
                zip.add_directory(rel.to_string_lossy(), opts).unwrap();
            }
        }
    } else {
        zip.start_file(source.file_name().unwrap().to_string_lossy(), opts).unwrap();
        let data = std::fs::read(source).unwrap();
        zip.write_all(&data).unwrap();
        count = 1;
    }
    zip.finish().unwrap();

    let size = std::fs::metadata(&out).map(|m| m.len()).unwrap_or(0);
    println!("{} Zipped {} file(s) → {}  ({})",
        "✓".green().bold(), count, out.display().to_string().cyan(),
        human_size(size).yellow());
}

// ─────────────────────────────────────────────────────────────
//  UNZIP
// ─────────────────────────────────────────────────────────────
fn cmd_unzip(archive: &Path, destination: &Path) {
    let file = std::fs::File::open(archive).expect("Cannot open archive");
    let mut zip = zip::ZipArchive::new(file).expect("Not a valid zip");
    std::fs::create_dir_all(destination).unwrap();
    let total = zip.len();
    zip.extract(destination).expect("Extraction failed");
    println!("{} Extracted {} files → {}",
        "✓".green().bold(), total, destination.display().to_string().cyan());
}

// ─────────────────────────────────────────────────────────────
//  RENAME
// ─────────────────────────────────────────────────────────────
fn cmd_rename(pattern: &str, replacement: &str, directory: &Path, dry_run: bool) {
    let mut count = 0usize;
    for entry in walkdir::WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() { continue; }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.contains(pattern) { continue; }

        let new_name = name.replace(pattern, replacement);
        let new_path = path.with_file_name(&new_name);
        let tag = if dry_run { "(dry-run)".dimmed().to_string() } else { String::new() };
        println!("  {}  →  {}  {}", name.yellow(), new_name.green(), tag);
        if !dry_run {
            std::fs::rename(path, new_path).ok();
        }
        count += 1;
    }
    if count == 0 {
        println!("{}", format!("No files matched pattern '{pattern}'").yellow());
    } else {
        let verb = if dry_run { "Would rename" } else { "Renamed" };
        println!("\n{} {verb} {count} file(s).", "✓".green().bold());
    }
}

// ─────────────────────────────────────────────────────────────
//  COUNT
// ─────────────────────────────────────────────────────────────
fn cmd_count(directory: &Path, ext_filter: Vec<String>) {
    use std::collections::HashMap;
    let filter: Vec<String> = ext_filter.iter().map(|e| {
        if e.starts_with('.') { e.clone() } else { format!(".{e}") }
    }).collect();

    let mut by_ext: HashMap<String, (usize, usize)> = HashMap::new();

    for entry in walkdir::WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        let ext = path.extension()
            .map(|e| format!(".{}", e.to_string_lossy().to_lowercase()))
            .unwrap_or_else(|| "(no ext)".into());

        if !filter.is_empty() && !filter.contains(&ext) { continue; }

        let lines = std::fs::read_to_string(path)
            .map(|s| s.lines().count())
            .unwrap_or(0);

        let entry = by_ext.entry(ext).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += lines;
    }

    if by_ext.is_empty() {
        println!("{}", "No files found.".yellow());
        return;
    }

    let mut rows: Vec<_> = by_ext.iter().collect();
    rows.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    println!("\n  {:<14} {:<10} {}", "EXTENSION".bold(), "FILES".bold(), "LINES".bold());
    let (mut tf, mut tl) = (0usize, 0usize);
    for (ext, (files, lines)) in &rows {
        println!("  {:<14} {:<10} {:>10}", ext.cyan(), files.to_string().yellow(), format!("{lines:>10}").white());
        tf += files; tl += lines;
    }
    println!("  {:<14} {:<10} {:>10}", "TOTAL".bold(), tf.to_string().bold(), format!("{tl:>10}").bold());
    println!();
}

// ─────────────────────────────────────────────────────────────
//  HASH
// ─────────────────────────────────────────────────────────────
fn cmd_hash(file: &Path) {
    use sha2::{Digest, Sha256};
    if !file.is_file() {
        eprintln!("{} Not a file: {}", "✗".red(), file.display());
        std::process::exit(1);
    }
    let data = std::fs::read(file).expect("Cannot read file");
    let md5_hash = format!("{:x}", md5::compute(&data));
    let sha256_hash = format!("{:x}", Sha256::digest(&data));
    let size = human_size(data.len() as u64);

    println!("\n  {} {}", "File:  ".bold(), file.display().to_string().cyan());
    println!("  {} {}", "Size:  ".bold(), size.yellow());
    println!("  {} {}", "MD5:   ".bold(), md5_hash.green());
    println!("  {} {}", "SHA256:".bold(), sha256_hash.green());
    println!();
}

// ─────────────────────────────────────────────────────────────
//  BACKUP
// ─────────────────────────────────────────────────────────────
fn cmd_backup(source: &Path, dest: &Path) {
    use chrono::Local;
    let stamp = Local::now().format("%Y%m%d_%H%M%S");
    let name = source.file_name().unwrap_or(std::ffi::OsStr::new("backup"))
        .to_string_lossy();
    std::fs::create_dir_all(dest).unwrap();
    let out = dest.join(format!("{name}_{stamp}.zip"));
    cmd_zip(source, Some(out.clone()));
    let size = std::fs::metadata(&out).map(|m| m.len()).unwrap_or(0);
    println!("{} Backup saved → {}  ({})",
        "✓".green().bold(), out.display().to_string().cyan(), human_size(size).yellow());
}

// ─────────────────────────────────────────────────────────────
//  ENV
// ─────────────────────────────────────────────────────────────
fn cmd_env(filter: &str) {
    let lower = filter.to_lowercase();
    let mut vars: Vec<(String, String)> = std::env::vars()
        .filter(|(k, v)| {
            filter.is_empty()
                || k.to_lowercase().contains(&lower)
                || v.to_lowercase().contains(&lower)
        })
        .collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    println!();
    for (k, v) in &vars {
        println!("  {}={}", k.cyan().bold(), v.yellow());
    }
    println!();
}

// ─────────────────────────────────────────────────────────────
//  HTTP
// ─────────────────────────────────────────────────────────────
fn cmd_http(port: u16, directory: &Path) {
    use std::net::TcpListener;
    use std::io::{Write, BufRead, BufReader};

    let dir = directory.canonicalize().unwrap_or(directory.to_path_buf());
    std::env::set_current_dir(&dir).unwrap();

    println!("{} Serving {} at {}",
        "⚡".yellow(),
        dir.display().to_string().yellow(),
        format!("http://localhost:{port}").cyan().bold(),
    );
    println!("{}", "Press Ctrl-C to stop\n".dimmed());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).expect("Failed to bind port");

    for mut stream in listener.incoming().flatten() {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut first_line = String::new();
            reader.read_line(&mut first_line).ok();

            let path_str = first_line
                .split_whitespace().nth(1).unwrap_or("/")
                .to_string();
            let decoded = percent_decode(&path_str);
            let rel_path = decoded.trim_start_matches('/');
            let full_path = if rel_path.is_empty() {
                dir.clone()
            } else {
                dir.join(rel_path)
            };

            let response = if full_path.is_dir() {
                let mut body = format!(
                    "<html><head><meta charset='utf-8'></head><body><h2>📁 {}</h2><ul>",
                    full_path.display()
                );
                if let Ok(entries) = std::fs::read_dir(&full_path) {
                    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                    items.sort_by_key(|e| e.file_name());
                    for item in items {
                        let n = item.file_name().to_string_lossy().to_string();
                        let href = format!("{}/{}", path_str.trim_end_matches('/'), n);
                        let icon = if item.path().is_dir() { "📁" } else { "📄" };
                        body.push_str(&format!("<li>{icon} <a href='{href}'>{n}</a></li>"));
                    }
                }
                body.push_str("</ul></body></html>");
                format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(), body)
            } else if full_path.is_file() {
                match std::fs::read(&full_path) {
                    Ok(data) => {
                        println!("  {} {}", "GET".green(), path_str);
                        let mime = guess_mime(full_path.extension()
                            .unwrap_or_default().to_str().unwrap_or(""));
                        let header = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                            mime, data.len()
                        );
                        stream.write_all(header.as_bytes()).ok();
                        stream.write_all(&data).ok();
                        continue;
                    }
                    Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
                }
            } else {
                "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\n404 oops".to_string()
            };

            stream.write_all(response.as_bytes()).ok();
    }
}

fn percent_decode(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&h, 16) {
                out.push(byte as char);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn guess_mime(ext: &str) -> &'static str {
    match ext {
        "html" | "htm" => "text/html",
        "css"          => "text/css",
        "js"           => "application/javascript",
        "json"         => "application/json",
        "png"          => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif"          => "image/gif",
        "svg"          => "image/svg+xml",
        "pdf"          => "application/pdf",
        "txt" | "md"   => "text/plain",
        _              => "application/octet-stream",
    }
}

// ─────────────────────────────────────────────────────────────
//  DIFF
// ─────────────────────────────────────────────────────────────
fn cmd_diff(dir_a: &Path, dir_b: &Path) {
    use std::collections::HashSet;

    fn all_files(root: &Path) -> HashSet<String> {
        walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().strip_prefix(root).unwrap().to_string_lossy().to_string())
            .collect()
    }

    let fa = all_files(dir_a);
    let fb = all_files(dir_b);

    let only_a: Vec<_> = fa.difference(&fb).collect();
    let only_b: Vec<_> = fb.difference(&fa).collect();
    let changed: Vec<_> = fa.intersection(&fb)
        .filter(|f| {
            let a = std::fs::read(dir_a.join(f)).unwrap_or_default();
            let b = std::fs::read(dir_b.join(f)).unwrap_or_default();
            a != b
        })
        .collect();

    if only_a.is_empty() && only_b.is_empty() && changed.is_empty() {
        println!("{}", "✓ Directories are identical.".green().bold());
        return;
    }

    println!("\n  Diff: {}  vs  {}\n",
        dir_a.display().to_string().cyan(),
        dir_b.display().to_string().cyan());

    for f in only_a  { println!("  {}  {}", "only in A".red(),     f.dimmed()); }
    for f in only_b  { println!("  {}  {}", "only in B".green(),   f.dimmed()); }
    for f in changed { println!("  {}   {}", "modified".yellow(), f.dimmed()); }
    println!();
}

// ─────────────────────────────────────────────────────────────
//  DUPE
// ─────────────────────────────────────────────────────────────
fn cmd_dupe(directory: &Path) {
    use std::collections::HashMap;

    let mut hashes: HashMap<String, Vec<PathBuf>> = HashMap::new();

    print!("Scanning… ");
    for entry in walkdir::WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.into_path();
        if let Ok(data) = std::fs::read(&path) {
            let hash = format!("{:x}", md5::compute(&data));
            hashes.entry(hash).or_default().push(path);
        }
    }

    let dupes: Vec<_> = hashes.values().filter(|v| v.len() > 1).collect();
    if dupes.is_empty() {
        println!("\n{}", "✓ No duplicates found!".green().bold());
        return;
    }

    let total: usize = dupes.iter().map(|v| v.len() - 1).sum();
    println!("\n{} group(s)  ({} extra copies):\n",
        dupes.len().to_string().yellow().bold(),
        total.to_string().red());

    let base = directory.canonicalize().unwrap_or(directory.to_path_buf());
    for group in &dupes {
        let size = group[0].metadata().map(|m| m.len()).unwrap_or(0);
        println!("  {} — {}", human_size(size).yellow(), format!("{} copies", group.len()).red());
        for p in *group {
            let rel = p.strip_prefix(&base).unwrap_or(p);
            println!("    {}", rel.display().to_string().cyan());
        }
        println!();
    }
}

// ─────────────────────────────────────────────────────────────
//  HELPERS
// ─────────────────────────────────────────────────────────────
fn human_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut s = size as f64;
    for unit in UNITS {
        if s < 1024.0 { return format!("{s:.1} {unit}"); }
        s /= 1024.0;
    }
    format!("{s:.1} PB")
}

fn confirm(msg: &str) -> bool {
    use std::io::Write;
    print!("{} [y/N] ", msg.yellow());
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

// ── Glob crate shim (we use the ignore crate for globs) ─────
mod glob {
    pub fn glob(pattern: &str) -> Result<impl Iterator<Item = Result<std::path::PathBuf, ()>>, ()> {
        let pat = pattern.to_string();
        let base = std::path::Path::new(".");
        let entries: Vec<_> = walkdir::WalkDir::new(base)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(move |e| {
                let name = e.file_name().to_string_lossy().to_string();
                super::glob_match(&pat, &name)
                    || e.path().to_string_lossy().contains(pat.trim_matches('*'))
            })
            .map(|e| Ok(e.into_path()))
            .collect();
        Ok(entries.into_iter())
    }
}
