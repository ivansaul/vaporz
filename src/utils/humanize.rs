pub fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, units[unit_index])
}

pub fn format_last_modified(secs: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;

    let (unit, fstring) = match secs {
        secs if secs < MINUTE => (secs as f64, "s"),
        secs if secs < HOUR => (secs as f64 / MINUTE as f64, "min"),
        secs if secs < DAY => (secs as f64 / HOUR as f64, "h"),
        _ => (secs as f64 / DAY as f64, "d"),
    };

    format!("{unit:.0}{fstring}")
}
