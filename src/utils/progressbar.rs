use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    spinner: ProgressBar,
}

impl Spinner {
    pub fn new() -> Spinner {
        let spinner = ProgressBar::new_spinner();

        spinner.enable_steady_tick(120);

        Spinner { spinner }
    }

    pub fn set_message(&self, msg: &'static str) {
        self.spinner.set_message(msg);
    }

    pub fn success(&self, msg: &'static str) {
        self.spinner
            .set_style(ProgressStyle::default_spinner().template("{msg:.green}"));
        self.spinner.finish_with_message(msg);
    }
}
