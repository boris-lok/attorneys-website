use crate::repositories::WorkLog;
use chrono::FixedOffset;
use rust_xlsxwriter::{Format, Workbook};

pub fn generate(logs: Vec<WorkLog>) -> anyhow::Result<Workbook> {
    let mut wb = Workbook::new();

    let ws = wb.add_worksheet();

    let date_formatter = Format::new().set_num_format("yyyy-mm-dd hh:mm");

    let mut cnt = 0;

    ws.write_string(cnt, 0, "工作開始時期")?;
    ws.write_string(cnt, 1, "工作結束時間")?;
    ws.write_string(cnt, 2, "時數")?;
    ws.write_string(cnt, 3, "工作人員")?;
    ws.write_string(cnt, 4, "工作內容概述")?;
    ws.write_string(cnt, 5, "總時數")?;

    cnt += 1;
    // convert to +8
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();

    let mut total_duration = 0.0;
    for log in logs {
        let mut collaborators = log
            .collaborators
            .into_iter()
            .filter(|c| c.status == "approved")
            .map(|c| c.name.clone())
            .collect::<Vec<String>>();
        collaborators.push(log.user.name.clone());
        collaborators.sort();

        let _duration = log.duration as f64 * collaborators.len() as f64;
        total_duration += _duration;
        let _duration = round_duration(_duration / 60.0);

        let users = collaborators.join(", ");

        ws.write_datetime_with_format(
            cnt,
            0,
            log.started_at.with_timezone(&tz).naive_local(),
            &date_formatter,
        )?;
        ws.write_datetime_with_format(
            cnt,
            1,
            log.ended_at.with_timezone(&tz).naive_local(),
            &date_formatter,
        )?;
        ws.write_number(cnt, 2, round_duration(log.duration as f64 / 60.0))?;
        ws.write_string(cnt, 3, users)?;
        ws.write_string(cnt, 4, log.description)?;
        ws.write_number(cnt, 5, _duration)?;

        cnt += 1;
    }

    ws.write_number(cnt, 5, round_duration(total_duration / 60.0))?;

    Ok(wb)
}

fn round_duration(duration: f64) -> f64 {
    (duration * 100.0).round() / 100.0
}
