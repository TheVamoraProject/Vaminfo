static TIPS: &[&str] = &[
    "Use `Ctrl+R` in bash/zsh to search your command history interactively.",
    "Press `!!` to repeat the last command, or `!$` to reuse its last argument.",
    "Use `&&` to chain commands: `cmd1 && cmd2` runs cmd2 only if cmd1 succeeds.",
    "`cd -` switches you back to the previous directory instantly.",
    "Use `man` pages! `man <command>` shows the full manual for any tool.",
    "Pipe to `tee` to write output to a file AND see it on screen: `cmd | tee output.txt`",
    "`Alt+.` in bash inserts the last argument of the previous command.",
    "Use `screen` or `tmux` to keep sessions alive after disconnecting from SSH.",
    "`watch -n 2 <command>` runs a command every 2 seconds — great for live monitoring.",
    "`find / -name '*.log' -mtime +30 -delete` removes log files older than 30 days.",
    "Use `df -h` for human-readable disk usage, and `du -sh *` to size directories.",
    "`grep -r 'pattern' .` recursively searches all files in the current directory.",
    "Use `chmod +x script.sh && ./script.sh` to make and run a script in one go.",
    "`curl -s https://wttr.in/<city>` shows the weather in your terminal.",
    "`history | awk '{print $2}' | sort | uniq -c | sort -rn | head` shows your most-used commands.",
    "Use `lsof -i :<port>` to find which process is using a specific port.",
    "`ssh -N -L 8080:localhost:80 user@remote` creates a local port tunnel.",
    "Use `xargs` to pass multiple arguments: `find . -name '*.tmp' | xargs rm`",
    "`set -euo pipefail` at the top of bash scripts makes them fail safely on errors.",
    "Use `systemctl status <service>` to check service health, or `journalctl -xe` for logs.",
    "`rsync -avz src/ dest/` is safer and faster than cp for syncing directories.",
    "Use `trap 'cleanup' EXIT` in scripts to run cleanup even if the script crashes.",
    "`cal` shows a calendar. `cal 2025` shows the full year. `ncal` for alternative layout.",
    "Use `diff -u file1 file2` for unified diffs. Pipe to `colordiff` for color output.",
    "Environment variables set with `export VAR=value` persist for the session.",
];

pub fn random_tip() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    TIPS[seed % TIPS.len()]
}
