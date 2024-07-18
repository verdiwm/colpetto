use std::ffi::CString;

use anyhow::Result as AnyResult;
use colpetto::Libinput;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let mut libinput = Libinput::new()?;
    libinput.assign_seat(CString::new("seat0").unwrap().as_c_str())?;

    while let Some(event) = libinput.try_next().await? {
        dbg!(event);
    }

    Ok(())
}
