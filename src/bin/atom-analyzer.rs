use std::path::PathBuf;

use atom_analyzer::qtfile::{self, QtFileError};
use clap::Clap;

#[cfg(feature = "gui")]
use qt_core::{qs, QBox, QPtr};
#[cfg(feature = "gui")]
use qt_widgets::{QAction, QApplication, QMainWindow, QMenu, QVBoxLayout};
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
    main_window: QBox<QMainWindow>,

    menu_file: QPtr<QMenu>,
}

#[cfg(feature = "gui")]
impl MainWindow {
    fn new() -> Rc<MainWindow> {
        let main_window = unsafe { QMainWindow::new_0a() };

        let menu_file = unsafe { main_window.menu_bar().add_menu_q_string(&qs("&File")) };

        let mut this = Self {
            main_window,
            menu_file,
        };

        this.init();

        Rc::new(this)
    }

    fn init(&mut self) {
        unsafe {
            let action_quit = QAction::from_q_string(&qs("Quit"));

            self.menu_file.add_action(&action_quit);
        }
    }

    fn show(&self) {
        unsafe { self.main_window.show() };
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
