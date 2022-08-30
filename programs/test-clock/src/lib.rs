use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod test_clock {
    use super::*;

    pub const UNIX_DAY: i64 = 86400;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {

        let clock = Clock::get().unwrap();
        
        let current_utc = convert_unix_to_utc(clock.unix_timestamp).unwrap();

        msg!("Current unix time: {}", clock.unix_timestamp);
        msg!("Current time in UTC: {:?}", current_utc);

        Ok(())
    }
}

pub fn convert_unix_to_utc(mut unix_time: i64) -> Result<UtcTime> {
    /* calculate minutes */
    let mut minutes = unix_time / 60;
    unix_time -= minutes * 60;
    /* calculate hours */
    let mut hours = minutes / 60;
    minutes -= hours * 60;
    /* calculate days */
    let mut days = hours / 24;
    hours -= days * 24;

     /* Unix time starts in 1970 on a Thursday */
    let mut year: i64 = 1970;
    let mut day_of_week: i64 = 4;
    let mut month: i64 = 0;

    loop {
        let mut leap_year: bool = false;
        let days_in_year: i64;
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { leap_year = true };
        if leap_year { days_in_year = 366 } else { days_in_year = 365 };
        if days >= days_in_year {
            if leap_year { day_of_week += 2 } else { day_of_week += 1 };
            days -= days_in_year;
            if day_of_week >= 7 {
                day_of_week -= 7;
            }
            year += 1;
        }
        else {
            //tm->tm_yday = days;
            day_of_week += days;
            day_of_week %= 7;

            /* calculate the month and day */
            let days_in_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            for i in 0..12 {
                let mut dim = days_in_month[i];
                /* add a day to feburary if this is a leap year */
                if i == 1 && leap_year {
                    dim += 1;
                }

                if days >= dim {
                    days -= dim;
                }
                else {
                    month = i as i64;
                    month += 1;
                    break;
                }
            }
            break;
        }
    }

    let current_time = UtcTime {
        day_of_week: day_of_week,
        sec: unix_time,
        min: minutes,
        hr: hours,
        day: days + 1,
        month: month,
        year: year
    };

    Ok(current_time)
}
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Debug)]
pub struct UtcTime {
    day_of_week: i64,
    sec: i64,
    min: i64,
    hr: i64,
    day: i64,
    month: i64,
    year: i64
}