mod request;
use std::collections::HashMap;
use std::marker::PhantomData;

use request::*;

pub struct ConcreteHandler<F, Args> {
    func: F,
    args: PhantomData<Args>,
}

pub trait IntoHandler<F, Args> {
    fn into_handler(self: Self) -> ConcreteHandler<F, Args>;
}

pub trait Handler {
    fn call(&self, args: &Request); // -> Self::Output;
}

macro_rules! factory_tuple_handler ({ $($param:ident)* } => {
    impl<Func, $($param:FromRequest,)*> Handler for ConcreteHandler<Func, ($($param,)*)>
    where
        Func: Fn($( $param, )*),
    {
        #[allow(unused_variables)]
        fn call(&self, request: &Request) {
            (self.func)($( $param::from_request(request), )*);
        }
    }
});

factory_tuple_handler! {}
factory_tuple_handler! { A }
factory_tuple_handler! { A B }
factory_tuple_handler! { A B C }
factory_tuple_handler! { A B C D }
factory_tuple_handler! { A B C D E }
factory_tuple_handler! { A B C D E F }
factory_tuple_handler! { A B C D E F G }
factory_tuple_handler! { A B C D E F G H }
factory_tuple_handler! { A B C D E F G H I }
factory_tuple_handler! { A B C D E F G H I J }
factory_tuple_handler! { A B C D E F G H I J K }
factory_tuple_handler! { A B C D E F G H I J K L }

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, $($param,)*> IntoHandler<Func, ($($param,)*)> for Func
    where
        $( $param: FromRequest, )*
        Func: Fn($($param),*)
    {
        // type Output = Box<dyn Debug>;

        #[inline]
        #[allow(non_snake_case)]
        fn into_handler(self: Func) -> ConcreteHandler<Func, ($($param,)*)> { // -> Self::Output {
            ConcreteHandler {
                func: self,
                args: PhantomData
            }
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

struct Router {
    routes: HashMap<String, Box<dyn Handler>>,
}

impl Router {
    fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    fn route<F, Args: 'static>(mut self, route: &str, handler: F) -> Self
    where
        F: IntoHandler<F, Args> + 'static,
        ConcreteHandler<F, Args>: Handler,
    {
        self.routes
            .insert(route.into(), Box::new(handler.into_handler()));
        self
    }

    fn call(&self, route: &str, args: &Request) {
        let handler = self.routes.get(route).unwrap();
        handler.call(args);
    }
}

struct Json<T>(T);

impl<T> FromRequest for Json<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from_request(req: &Request) -> Self
    where
        Self: Sized,
    {
        Json(serde_json::from_str(&req.0).unwrap())
    }
}

#[derive(serde::Deserialize)]
struct MyJson {
    f_string: String,
    f_f32: f32,
}

fn test_json(Json(my_json): Json<MyJson>) {
    println!("s: {:?}, f32: {:?}", my_json.f_string, my_json.f_f32);
}

fn test_mix(a: usize, b: String, c: f32) {
    println!("{:?}, {:?}, {:?}", a, b, c);
}

fn test_usize(a: usize) {
    println!("{}", a);
}

fn test_string(a: String) {
    println!("{:?}", a);
}

fn main() {
    let request = Request("42".into());
    let router = Router::new()
        .route("test_usize", test_usize)
        .route("test_string", test_string)
        .route("test_mix", test_mix)
        .route("test_json", test_json);

    router.call("test_usize", &request);
    router.call("test_string", &request);
    router.call("test_mix", &request);

    let request = Request(
        r#"{
            "f_string": "the cake is a lie",
            "f_f32": 42.0
        }"#
        .into(),
    );
    router.call("test_json", &request);
}
