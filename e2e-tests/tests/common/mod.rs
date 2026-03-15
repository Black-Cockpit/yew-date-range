//! Shared test infrastructure for end-to-end browser tests.
//!
//! Provides a reusable fixture that starts an Actix static file server
//! and a headless Chromium browser via Playwright, giving each test
//! a fresh page navigated to the example application.

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use playwright::Playwright;
use playwright::api::browser::Browser;
use playwright::api::browser_context::BrowserContext;
use playwright::api::page::Page;
use std::net::TcpListener;
use std::path::PathBuf;
use tokio::sync::OnceCell;

/// Shared test infrastructure holding one server and one browser per test file.
///
/// Fields prefixed with `_` are kept alive for the duration of the fixture
/// but are not accessed directly by tests.
pub struct SharedFixture {
    /// The TCP port the static file server is listening on.
    pub port: u16,

    /// Handle to the running Actix server (kept alive, not accessed directly).
    pub _server_handle: actix_web::dev::ServerHandle,

    /// The Playwright instance (kept alive, not accessed directly).
    pub _pw: Playwright,

    /// The Chromium browser instance (kept alive, not accessed directly).
    pub _browser: Browser,

    /// The browser context used to create fresh pages for each test.
    pub context: BrowserContext,
}

/// Finds an available TCP port on localhost by binding to port 0.
///
/// # Returns
///
/// - `Ok(u16)`: An available port number.
///
/// # Errors
///
/// - Returns an IO error if binding fails.
fn available_port() -> Result<u16, std::io::Error> {
    // Bind to port 0 and let the OS assign an available port.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    Ok(listener.local_addr()?.port())
}

/// Resolves the absolute path to a dist directory from a workspace-relative path.
///
/// # Parameters
///
/// - `relative`: The path relative to the workspace root (e.g., "example/dist").
///
/// # Returns
///
/// - The absolute path to the dist directory.
fn dist_path(relative: &str) -> PathBuf {
    // Navigate from the crate manifest directory up to the workspace root.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(|p| p.join(relative))
        .unwrap_or_else(|| manifest.join(relative))
}

/// Creates the shared fixture with one Actix server and one Playwright browser.
///
/// # Parameters
///
/// - `dist_relative`: The workspace-relative path to the built dist directory.
///
/// # Returns
///
/// - `Ok(SharedFixture)`: The initialized test infrastructure.
///
/// # Errors
///
/// - Returns an error if the server fails to bind or the browser fails to launch.
pub async fn create_fixture(dist_relative: &str) -> Result<SharedFixture, Box<dyn std::error::Error + Send + Sync>> {
    // Resolve the dist directory absolute path.
    let dist = dist_path(dist_relative);

    // Find an available port for the static file server.
    let port = available_port()?;
    let dist_str = dist.to_string_lossy().to_string();

    // Configure and start the Actix static file server.
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(Files::new("/", &dist_str).index_file("index.html").prefer_utf8(true))
    })
    .bind(("127.0.0.1", port))?
    .disable_signals()
    .run();

    // Extract the server handle for graceful shutdown.
    let server_handle = server.handle();

    // Spawn the server on a dedicated thread with its own Actix runtime.
    std::thread::spawn(move || {
        let rt = actix_web::rt::System::new();
        rt.block_on(server).ok();
    });

    // Wait for the server to become ready.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Initialize Playwright and install Chromium if needed.
    let pw = Playwright::initialize().await?;
    pw.install_chromium()?;

    // Launch headless Chromium with WASM experimental features enabled.
    let chromium = pw.chromium();
    let browser = chromium
        .launcher()
        .headless(false)
        .args(&[
            String::from("--enable-features=WebAssemblyGC,WebAssemblyExperimentalJSPI"),
            String::from("--enable-experimental-webassembly-features"),
        ])
        .launch()
        .await?;

    // Create a browser context for isolating test pages.
    let context = browser.context_builder().build().await?;

    Ok(SharedFixture {
        port,
        _server_handle: server_handle,
        _pw: pw,
        _browser: browser,
        context,
    })
}

/// Creates a fresh browser page from the shared fixture, navigated and ready.
///
/// Lazily initializes the fixture on first call, then reuses the same server
/// and browser for subsequent calls within the same test file.
///
/// # Parameters
///
/// - `cell`: The once-cell holding the shared fixture.
/// - `dist_relative`: The workspace-relative path to the built dist directory.
///
/// # Returns
///
/// - `Ok(Page)`: A new Playwright page with the example loaded and the first
///   calendar widget visible.
///
/// # Errors
///
/// - Returns an error if fixture initialization or page navigation fails.
pub async fn new_page(
    cell: &OnceCell<SharedFixture>,
    dist_relative: &str,
) -> Result<Page, Box<dyn std::error::Error + Send + Sync>> {
    // Initialize the fixture lazily on first access.
    let fixture = cell.get_or_try_init(|| create_fixture(dist_relative)).await?;

    // Open a new browser page.
    let page = fixture.context.new_page().await?;

    // Navigate to the example application.
    let url = format!("http://127.0.0.1:{}", fixture.port);
    page.goto_builder(&url).goto().await?;

    // Wait until the first calendar widget is fully rendered.
    page.main_frame()
        .wait_for_selector_builder(".rdrCalendarWrapper")
        .wait_for_selector()
        .await?;

    Ok(page)
}
