pub mod ui {
    use indicatif::{ProgressBar as IndicatifProgressBar, ProgressStyle};
    use colored::Colorize;

    pub struct ProgressBar {
        inner: IndicatifProgressBar,
    }

    impl ProgressBar {
        pub fn new(len: u64) -> Self {
            let inner = IndicatifProgressBar::new(len);
            inner.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                    .unwrap()
                    .progress_chars("#>-")
            );
            Self { inner }
        }

        pub fn inc(&self, delta: u64) {
            self.inner.inc(delta);
        }

        pub fn finish_with_message(&self, msg: &str) {
            self.inner.finish_with_message(msg.green().to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pb = ui::ProgressBar::new(10);
        pb.inc(1);
        pb.finish_with_message("Done");
    }
}
