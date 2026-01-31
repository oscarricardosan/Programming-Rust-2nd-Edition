use std::collections::HashMap;

struct Request{
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response{
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

type BoxedCallback= Box<dyn Fn(&Request) -> Response>;
struct BasicRouter{
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter{

    //Crea un enrutador vácio 
    fn new()-> BasicRouter{
        BasicRouter{routes: HashMap::new()}
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

}

impl BasicRouter {

    fn handle_request(&self, request: &Request) -> Response{
        match self.routes.get(&request.url) {
            None=> not_found_response(),
            Some(callback)=> callback(request),
        }
    }

}

fn main() {

    let mut router= BasicRouter::new();
    router.add_route("/", |_|get_form_response());

    println!("✅ Finalizado!");
}

fn get_form_response(){
    println!("Ok");
}
