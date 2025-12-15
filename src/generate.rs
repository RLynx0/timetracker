use std::{collections::HashMap, rc::Rc};

use chrono::{DateTime, Datelike, Local, NaiveDate, TimeDelta};

use crate::{activity_entry::TrackedActivity, config::Config};

/// Grouping of activities with
/// - Same wbs
/// - Same description
/// - Same attendance type
/// - Same local date (precise time is irrelevant)
#[derive(Debug, Clone)]
struct CollapsedActivity {
    attendance_type: Rc<str>,
    description: Rc<str>,
    duration: TimeDelta,
    start_of_first: DateTime<Local>,
    wbs: Rc<str>,
}

fn collapse_activities(
    activities: &[TrackedActivity],
    end_fallback: DateTime<Local>,
) -> Vec<CollapsedActivity> {
    let mut grouped_activities = HashMap::new();
    let activities: Vec<_> = activities
        .iter()
        .cloned()
        .flat_map(|t| t.split_on_midnight(end_fallback))
        .collect();
    for activity in &activities {
        grouped_activities
            .entry(get_group_key(activity))
            .or_insert_with(|| CollapsedActivity {
                attendance_type: activity.attendance().into(),
                description: activity.description().into(),
                duration: TimeDelta::zero(),
                start_of_first: *activity.start_time(),
                wbs: activity.wbs().into(),
            })
            .duration +=
            activity.end_time().copied().unwrap_or(Local::now()) - activity.start_time();
    }

    let mut grouped_activities = Vec::from_iter(grouped_activities.into_values());
    grouped_activities.sort_unstable_by(|a, b| a.start_of_first.cmp(&b.start_of_first));
    grouped_activities
}

#[derive(PartialEq, Eq, Hash)]
struct ActivityGroupKey<'a> {
    wbs: &'a str,
    attendance_type: &'a str,
    description: &'a str,
    date: NaiveDate,
}

fn get_group_key<'a>(activity: &'a TrackedActivity) -> ActivityGroupKey<'a> {
    ActivityGroupKey {
        wbs: activity.wbs(),
        attendance_type: activity.attendance(),
        description: activity.description(),
        date: activity.start_time().naive_local().date(),
    }
}

fn vars_from_config(cfg: &Config) -> HashMap<&'static str, &str> {
    HashMap::from([
        ("employee_name", cfg.employee_name.as_str()),
        ("employee_number", cfg.employee_number.as_str()),
        ("cost_center", cfg.cost_center.as_str()),
        ("performance_type", cfg.performance_type.as_str()),
        ("accounting_cycle", cfg.accounting_cycle.as_str()),
    ])
}

fn vars_from_collapsed_activity(activity: &CollapsedActivity) -> HashMap<&'static str, Rc<str>> {
    let date = activity.start_of_first.naive_local().date();
    let seconds = activity.duration.as_seconds_f64();
    HashMap::from([
        // Regarding date
        ("year", Rc::from(date.year().to_string())),
        ("month", Rc::from(date.month().to_string())),
        ("day", Rc::from(date.year().to_string())),
        // Regarding duration
        ("hours", Rc::from((seconds / 3600.0).to_string())),
        ("minutes", Rc::from((seconds / 60.0).to_string())),
        ("seconds", Rc::from(seconds.to_string())),
        // Other
        ("attendance_type", activity.attendance_type.clone()),
        ("description", activity.description.clone()),
        ("wbs", activity.wbs.clone()),
    ])
}
