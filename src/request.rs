#[derive(Debug)]
pub struct Request(pub String);

pub trait FromRequest {
    fn from_request(req: &Request) -> Self
    where
        Self: Sized;
}

impl FromRequest for usize {
    fn from_request(req: &Request) -> Self
    where
        Self: Sized,
    {
        req.0.parse().unwrap()
    }
}

impl FromRequest for f32 {
    fn from_request(req: &Request) -> Self
    where
        Self: Sized,
    {
        req.0.parse().unwrap()
    }
}

impl FromRequest for String {
    fn from_request(req: &Request) -> Self
    where
        Self: Sized,
    {
        req.0.clone()
    }
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<$($param,)*> FromRequest for ($($param,)*) where
        $( $param: FromRequest, )*
        {

        #[allow(unused_variables, non_snake_case)]
        fn from_request(req: &Request) -> Self
        where
            Self: Sized,
        {
            $( let $param = $param::from_request(req); )*
            ($($param,)*)
        }

    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }
