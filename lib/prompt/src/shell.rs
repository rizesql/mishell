use std::sync::Arc;

use ast::executables_cache::EXECUTABLES_CACHE;
use mishell_core::exec;
use nu_ansi_term::{Color, Style};
use reedline::MenuBuilder;
use tokio::sync::Mutex;

use crate::{
    control::TerminalControl,
    error::Error,
    refs::{self, EngineRef},
};

pub struct Mishell {
    reedline: reedline::Reedline,
    engine: EngineRef,
    terminal_control: TerminalControl,
}

enum ExecutionResult {
    Executed(exec::ExitCode),
    Failed(mishell_core::Error),
    Eof,
}

impl Mishell {
    pub fn new() -> Result<Self, Error> {
        let engine = mishell_core::Engine::new()?;

        let engine_ref = Arc::new(Mutex::new(engine));

        let completion_menu = Box::new(
            reedline::ColumnarMenu::default()
                .with_name("completion_menu")
                .with_marker("")
                .with_columns(10)
                .with_selected_text_style(Color::Blue.bold().reverse())
                .with_selected_match_text_style(Color::Blue.bold().reverse()),
        );

        let hinter = reedline::DefaultHinter::default()
            .with_style(Style::new().italic().fg(Color::DarkGray));

        let reedline = reedline::Reedline::create()
            .with_ansi_colors(true)
            .use_bracketed_paste(true)
            .with_quick_completions(true)
            .with_hinter(Box::new(hinter))
            .with_menu(reedline::ReedlineMenu::EngineCompleter(completion_menu));

        Ok(Self {
            reedline,
            engine: engine_ref,
            terminal_control: TerminalControl::acquire()?,
        })
    }

    pub fn engine(&self) -> impl AsRef<mishell_core::Engine> + Send + use<'_> {
        refs::EngineReader {
            engine: self.engine.try_lock().unwrap(),
        }
    }

    fn engine_mut(&mut self) -> impl AsMut<mishell_core::Engine> + Send + use<'_> {
        refs::EngineWriter {
            engine: self.engine.try_lock().unwrap(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.run_once()? {
                ExecutionResult::Executed(res) => {
                    if res.should_exit() {
                        break;
                    }
                }
                ExecutionResult::Failed(err) => tracing::error!("error: {:#}", err),
                ExecutionResult::Eof => break,
            }
        }

        Ok(())
    }

    #[tokio::main]
    async fn run_once(&mut self) -> Result<ExecutionResult, Error> {
        // Aici se face cache la executabilele din path
        let cache = EXECUTABLES_CACHE.clone();
        tokio::spawn(async move {
            cache.populate_cache().await;
        });

        match self.reedline.read_line(&reedline::DefaultPrompt::default()) {
            Ok(reedline::Signal::Success(cmd)) => {
                tracing::info!("{cmd}");

                self.engine_mut().as_mut().run(cmd).await?;

                Ok(ExecutionResult::Executed(exec::ExitCode::success()))
            }
            Ok(reedline::Signal::CtrlC) => {
                self.engine_mut().as_mut().set_last_exit_status(130);
                Ok(ExecutionResult::Executed(exec::ExitCode::new(130)))
            }
            Ok(reedline::Signal::CtrlD) => Ok(ExecutionResult::Eof),
            Err(err) => Err(Error::Io(err)),
        }
    }
}
