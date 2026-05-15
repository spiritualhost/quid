// Build app with icon
use std::io;
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_icon("assets/quid.ico");
        res.compile()?;
    }
    Ok(())
}