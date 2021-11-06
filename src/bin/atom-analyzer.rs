use std::path::PathBuf;

use atom_analyzer::qtfile::{self, QtFileError};
use clap::Clap;

#[cfg(feature = "gui")]
use qt_core::QBox;
#[cfg(feature = "gui")]
use qt_widgets::{QApplication, QWidget};
#[cfg(feature = "gui")]
use std::rc::Rc;

#[derive(Clap)]
#[clap(name=env!("CARGO_PKG_NAME"))]
struct Opts {
    #[clap(name = "INPUT")]
    input: PathBuf,
}

#[cfg(feature = "gui")]
struct MainWindow {
    widget: QBox<QWidget>,
}

#[cfg(feature = "gui")]
impl MainWindow {
    fn new() -> Rc<MainWindow> {
        let widget = unsafe { QWidget::new_0a() };

        Rc::new(Self { widget })
    }

    fn show(&self) {
        unsafe { self.widget.show() };
    }
}

#[cfg(feature = "gui")]
fn run(_file: &qtfile::QtFile) {
    QApplication::init(|_| unsafe {
        let main = MainWindow::new();

        main.show();

        QApplication::exec()
    })
}

#[cfg(not(feature = "gui"))]
fn run(file: &qtfile::QtFile) {
    println!("{:#?}", file);
}

fn main() -> Result<(), QtFileError> {
    let opts = Opts::parse();

    let t = qtfile::parse_file(opts.input)?;

    run(&t);

    Ok(())
}
