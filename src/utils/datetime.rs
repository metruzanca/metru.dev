pub fn format_date_range(start: &str, end: &str) -> String {
    let start_fmt = format_short_date(start);
    let end_fmt = if end.is_empty() {
        "Present".to_string()
    } else {
        format_short_date(end)
    };
    format!("{start_fmt} \u{2013} {end_fmt}")
}

pub fn format_short_date(iso: &str) -> String {
    if iso.len() < 7 {
        return iso.to_string();
    }
    let month = match &iso[5..7] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => "",
    };
    let year = &iso[0..4];
    format!("{month} {year}")
}
