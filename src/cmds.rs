#[path = "utils.rs"] mod utils;

pub fn new(days: &str) {
    let extra_days: i32 = days.parse().unwrap();
    let (month, day) = utils::get_date();
    let mut day: i32 = day.parse().unwrap();
    day += extra_days;

    let preserved_tasks = utils::preserve_tasks(&format!("./notes/{}/{}.md", month, day - 1));

    let month_path = format!("./notes/{}", month);
    let day_path = format!("./notes/{}/{}.md", month, day);

    if  utils::file_exists(&month_path) {
        utils::create_dir(&month_path);
    }

    println!("Creating file: {}", day_path);
    utils::create_file(&day_path, &preserved_tasks);
}

pub fn init() {
    if utils::file_exists("./notes") {
        println!("Notes directory already exists");
        return;
    }

    utils::create_dir("./notes");

    let (month, day) = utils::get_date();
    let month_path = format!("./notes/{}", month);
    let day_path = format!("./notes/{}/{}.md", month, day);
    utils::create_dir(&month_path);
    utils::create_file(&day_path, &Vec::new());
}