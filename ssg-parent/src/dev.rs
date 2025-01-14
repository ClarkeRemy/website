mod app;

use futures::FutureExt;
use reactive::driver::Driver;

/// Error type returned from the reactive app
#[derive(Debug, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum DevError {
    #[error(transparent)]
    Notify(#[from] reactive::driver::notify::Error),
    #[error(transparent)]
    Io(std::io::Error),
    #[error("no free port")]
    NoFreePort,
}

const BUILDER_CRATE_NAME: &str = "builder";
const LOCALHOST: &str = "localhost";

fn local_url(port: portpicker::Port) -> reqwest::Url {
    reqwest::Url::parse(&format!("http://{LOCALHOST}:{port}")).expect("valid")
}

/// Sets up a development environment that watches the file system,
/// recompiling the crate that when run describes the website on localhost when there are changes.
pub async fn dev<O>(launch_browser: bool, output_dir: O) -> DevError
where
    camino::Utf8PathBuf: From<O>,
{
    let output_dir = camino::Utf8PathBuf::from(output_dir);
    let Some(port) = portpicker::pick_unused_port() else {
        return DevError::NoFreePort;
    };

    let server_task = live_server::listen(LOCALHOST, port, output_dir.as_std_path().to_owned())
        .map(|result| result.expect_err("unreachable"))
        .boxed();

    let mut cargo_run_builder = tokio::process::Command::new("cargo");
    cargo_run_builder.args(["run", "--package", BUILDER_CRATE_NAME]);

    let url = local_url(port);

    let (builder_driver, builder_started) =
        reactive::driver::command::StaticCommandDriver::new(cargo_run_builder);
    let (child_process_killer_driver, child_killed) =
        reactive::driver::child_process_killer::ChildProcessKillerDriver::new();
    let (open_browser_driver, browser_opened) =
        reactive::driver::open_that::StaticOpenThatDriver::new(url.to_string());
    let (eprintln_driver, _) = reactive::driver::println::EprintlnDriver::new();
    let (notify_driver, notify) =
        match reactive::driver::notify::FsChangeDriver::new(BUILDER_CRATE_NAME) {
            Ok(val) => val,
            Err(e) => return e.into(),
        };

    let inputs = app::Inputs {
        server_task,
        child_killed,
        notify,
        builder_started,
        launch_browser,
        browser_opened,
        url,
    };

    let outputs = app::App::default().outputs(inputs);

    let app::Outputs {
        stderr,
        open_browser,
        error,
        kill_child,
        run_builder,
        stream_splitter_task,
    } = outputs;

    let builder_driver_task = builder_driver.init(run_builder);
    let child_process_killer_driver_task = child_process_killer_driver.init(kill_child);
    let open_browser_driver_task = open_browser_driver.init(open_browser);
    let stderr_driver_task = eprintln_driver.init(stderr);
    let notify_driver_task = notify_driver.init(());

    futures::select! {
        error = error.fuse() => error,
        _ = builder_driver_task.fuse() => unreachable!(),
        _ = child_process_killer_driver_task.fuse() => unreachable!(),
        _ = stderr_driver_task.fuse() => unreachable!(),
        _ = open_browser_driver_task.fuse() => unreachable!(),
        _ = stream_splitter_task.fuse() => unreachable!(),
        _ = notify_driver_task.fuse() => unreachable!(),
    }
}
