// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



use core::time;
use std::{rc::Rc, thread};

use chrono::{offset, DateTime, Local, NaiveDate, Utc};
mod clock;
use clock::{Clock, SystemClock};
use slint::{Model, ModelRc, SharedString, Timer, TimerMode, VecModel};

pub type EventResult = slint::private_unstable_api::re_exports::EventResult;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let timer = Timer::default();
    let ui = AppWindow::new()?;
    fn update_combo_box(mut index: i32, weak: slint::Weak<AppWindow>) {
        println!("Update");
        let ui = weak.upgrade().unwrap();
        if (index == 0) {index = 1;};
        if (index == 23) {index = 22};
        ui.set_integer(((index-1) * -240) as f32);

    }
    let local_offset_milli = Local::now().offset().local_minus_utc();
    let local_offset_hours: i32 = local_offset_milli / 3600;
    let prefix = "UTC+".to_owned();
    ui.set_timezone((prefix + &local_offset_hours.to_string()).into()); 
    let mut index = 11 + local_offset_hours;
    ui.set_selected(index);
    update_combo_box(index, ui.as_weak());
    let weak = ui.as_weak();
    ui.on_scrolled(move |sa|{
        println!("{:?}", sa);
        let up_ui = weak.clone().upgrade().unwrap();
        let mut new_delta = up_ui.get_integer() + sa.delta_y.signum()*240.0;
        if (new_delta < -5040.0) {new_delta = -5040.0};
        if (new_delta > 0.0) {new_delta = 0.0};
        
        up_ui.set_integer(new_delta);
        println!("{:?}", up_ui.get_integer());
        EventResult::Accept
    });

    let weak = ui.as_weak();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {

        let up_ui = weak.upgrade().unwrap();
        //let something: Result<_,_> = ModelRc::try_into(up_ui.get_all_times());
        let mut time_vector: Vec<slint::SharedString> = vec![
            "00:00".into();
            24
        ];
        let mut date_vector: Vec<slint::SharedString> = vec![
            "00:00".into();
            24
        ];
        let utc = SystemClock::now(Utc);
        let now = Local::now();

        let offset = -11;
        let binding = utc.to_string();
        let full_date = binding.split(".").next().unwrap();
        let mut full_date_split = full_date.split(" ");
        let date_const = full_date_split.next().unwrap();
        let mut date = date_const;
        let time = full_date_split.next().unwrap();
        let mut new_date;
        let selected_timezone_offset = up_ui.get_selected() -11 - local_offset_hours;
        println!("{:?}", selected_timezone_offset);
        println!("{:?}", local_offset_hours);
        for i in 0..24 {
            let mut time_values = time.split(":");
            let mut hour: i32 = time_values.next().unwrap().parse().unwrap();
        
        
            hour = hour + (offset + i) - selected_timezone_offset;
            if hour >= 24 {
                hour = hour - 24;
                new_date = add_day(date);
                date = new_date.as_str();

            };
            if hour < 0 {
                hour = hour + 24;
                new_date = remove_day(date);
                date = new_date.as_str();
            };

            

            let new_time = hour.to_string().to_owned() + ":" + time_values.next().unwrap() + ":" + time_values.next().unwrap();
            let usize_i: usize  = i.try_into().unwrap();
            time_vector[usize_i] = new_time.into();
            date_vector[usize_i] = date.into();
            date = date_const;
        }

        let time_model : Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(time_vector));
        let date_model : Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(date_vector));

        let time_model_rc = ModelRc::from(time_model.clone());
        let date_model_rc = ModelRc::from(date_model.clone());

        up_ui.set_all_times(time_model_rc);
        up_ui.set_all_dates(date_model_rc);
       // println!("{:?}", ());
       // let time_this = Local::now().time().to_string();
       // let mut clock_time = time_this.split(".");


        //up_ui.set_all_times(table);


    });
    let weak = ui.as_weak();
    pub fn get_days_from_month(year: i32, month: u32) -> i64 {
        NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            }, 
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        ).unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days()
    }
    fn add_day(
        date: &str
    ) -> String { 
        //println!("test: {:?}", date);
        let mut split_date = date.split("-");
        let mut year: i32 = split_date.next().unwrap().parse().unwrap();
        let mut month: i32 = split_date.next().unwrap().parse().unwrap();
        let mut day: i32 = split_date.next().unwrap().parse().unwrap();
        let max_days_64: i64 = get_days_from_month(year, month.unsigned_abs());
        let max_days: i32 = max_days_64.try_into().unwrap(); 

        if day== max_days {
            if month == 12 {
                day = 1;
                month = 1;
                year = year + 1;

            } else {
                day = 1;
                month = month + 1;

            } 

         } else { day = day + 1;

        }
        let mut new_month = month.to_string();
        if new_month.len() == 1 {
            new_month = "0".to_owned() + &new_month
        }
        let new_date: String = year.to_string().to_owned() + "-" + &new_month + "-" + &day.to_string();
        return new_date;
 
    }
    fn remove_day(
        date: &str
    ) -> String { 

        let mut split_date = date.split("-");
        let mut year: i32 = split_date.next().unwrap().parse().unwrap();
        let mut month: i32 = split_date.next().unwrap().parse().unwrap();
        let mut day: i32 = split_date.next().unwrap().parse().unwrap();

        println!("{:?} {:?} {:?} ", year, month, day);
        if day== 0 {
            if month == 0 {
                day = 1;
                month = 1;
                year = year - 1;
                println!("add year");
            } else {
                day = 1;
                month = month - 1;
                println!("add month");
            } 

         } else { day = day - 1;
            println!("add day");
        }
        let mut new_month = month.to_string();
        if new_month.len() == 1 {
            new_month = "0".to_owned() + &new_month
        }
        let new_date: String = year.to_string().to_owned() + "-" + &new_month + "-" + &day.to_string();
        return new_date;
 
    };

    ui.on_request_timezone(move |value| {
        let up_ui = weak.upgrade().unwrap();
        println!("{:?} {:?}",value, up_ui.get_selected());

    });
    let weak = ui.as_weak();
    ui.on_updatePosition(move || {
        let up_ui = weak.upgrade().unwrap();
        update_combo_box(up_ui.get_selected(), up_ui.as_weak());
    });

    ui.run()


}
