pub struct SevenDayItem {
    pub start_date: i64,
    pub end_date: i64,
    pub offset: i64,
}

pub fn get_seven_day() -> Vec<SevenDayItem> {
    let now = chrono::Utc::now().timestamp();
    let one_day_stamp = 24 * 60 * 60;
    let mut sevent_day_vec: Vec<SevenDayItem> = Vec::new();
    (0..7).for_each(|i| {
        let item = SevenDayItem {
            start_date: now - i * one_day_stamp,
            end_date: now - (i + 1) * one_day_stamp,
            offset: i,
        };
        sevent_day_vec.push(item);
    });
    sevent_day_vec
}
