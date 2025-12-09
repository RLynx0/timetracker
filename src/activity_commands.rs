use std::{fs, io::Write};

use color_eyre::{
    Section,
    eyre::{Result, format_err},
};
use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::char,
    combinator::{opt, recognize},
    multi::many0,
};

use crate::{files, opt, trackable::Activity};

pub fn set_activity(set_opts: &opt::SetActivity) -> Result<()> {
    let mut path = files::get_activity_dir_path()?;
    path.push(&set_opts.name);
    if path.is_dir() {
        return Err(format_err!("{path:?} is an activity category"));
    }
    if !set_opts.force && fs::exists(&path)? {
        return Err(format_err!("{path:?} already exists")
            .with_note(|| "Use --force to overwrite existing activities"));
    }
    if let Some(p) = path.parent() {
        fs::create_dir_all(p);
    }
    let activity = Activity::new(
        &set_opts.name,
        &set_opts.wbs,
        set_opts.description.as_deref(),
    );
    let mut file = fs::OpenOptions::new().create(true).write(true).open(path)?;
    writeln!(file, "{activity}")?;
    println!("Written {}", activity.name());
    Ok(())
}
