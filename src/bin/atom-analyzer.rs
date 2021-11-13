use std::path::PathBuf;

use atom_analyzer::qtfile::{self, QtFileError};
use clap::Clap;

#[cfg(feature = "gui")]
use cpp_core::{Ptr, StaticUpcast};
#[cfg(feature = "gui")]
use qt_core::{qs, slot, QBox, QObject, QPtr, SlotNoArgs};
#[cfg(feature = "gui")]
use qt_widgets::{QAction, QApplication, QMainWindow, QMenu};
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
    action_quit: QBox<QAction>,
}

#[cfg(feature = "gui")]
impl StaticUpcast<QObject> for MainWindow {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.main_window.as_ptr().static_upcast()
    }
}

#[cfg(feature = "gui")]
impl MainWindow {
    fn new() -> Rc<MainWindow> {
        let this = unsafe {
            let main_window = QMainWindow::new_0a();
            let menu_file = main_window.menu_bar().add_menu_q_string(&qs("&File"));
            let action_quit = QAction::from_q_string(&qs("E&xit"));

            main_window.show();

            Rc::new(Self {
                main_window,
                menu_file,
                action_quit,
            })
        };

        unsafe {
            this.init();
        }

        this
    }

    unsafe fn init(self: &Rc<Self>) {
        self.menu_file.add_separator();
        self.menu_file.add_action(&self.action_quit);

        self.action_quit.triggered().connect(&self.slot_on_quit());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_quit(self: &Rc<Self>) {
        self.main_window.close();
    }
}

#[cfg(feature = "gui")]
fn run(_file: &qtfile::QtFile) {
    QApplication::init(|_| unsafe {
        let _main = MainWindow::new();
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
