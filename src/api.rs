use leptos::{server, server_fn::ServerFnError};

#[server]
pub async fn get_counter() -> Result<i32, ServerFnError> {
    use crate::state::counter;

    let res = counter().await?.get();

    Ok(res)
}

#[server(IncCounter)]
pub async fn inc_counter() -> Result<(), ServerFnError> {
    use crate::state::counter;

    counter().await?.inc();

    Ok(())
}

#[server(DecCounter)]
pub async fn dec_counter() -> Result<(), ServerFnError> {
    use crate::state::counter;

    counter().await?.dec();

    Ok(())
}
