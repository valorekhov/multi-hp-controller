pub mod yield_now;

pub use yield_now::*;

pub fn into_poll<T, E>(r: nb::Result<T, E>)
    -> ::core::task::Poll<::core::result::Result<T, E>>
{
    match r {
        Err(nb::Error::Other(e)) => ::core::task::Poll::Ready(core::result::Result::Err(e)),
        Err(nb::Error::WouldBlock) => ::core::task::Poll::Pending,
        Ok(x) => ::core::task::Poll::Ready(core::result::Result::Ok(x)),
    }
}

pub fn as_poll<T, E>(r: &nb::Result<T, E>)
    -> ::core::task::Poll<::core::result::Result<&T, &E>>
{
    match r {
        Err(nb::Error::Other(ref e)) => ::core::task::Poll::Ready(core::result::Result::Err(e)),
        Err(nb::Error::WouldBlock) => ::core::task::Poll::Pending,
        Ok(ref x) => ::core::task::Poll::Ready(core::result::Result::Ok(x)),
    }
}

pub fn as_mut_poll<T, E>(r: &mut nb::Result<T, E>)
    -> ::core::task::Poll<::core::result::Result<&mut T, &mut E>>
{
    match r {
        Err(nb::Error::Other(ref mut e)) => ::core::task::Poll::Ready(core::result::Result::Err(e)),
        Err(nb::Error::WouldBlock) => ::core::task::Poll::Pending,
        Ok(ref mut x) => ::core::task::Poll::Ready(core::result::Result::Ok(x)),
    }
}