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

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>
}

impl FnPointerRouter{

    //Crea un enrutador vácio 
    fn new()-> FnPointerRouter{
        FnPointerRouter{routes: HashMap::new()}
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response)
    {
        self.routes.insert(url.to_string(), callback);
    }

    fn handle_request(&self, request: &Request) -> Response{
        match self.routes.get(&request.url) {
            None=> not_found_response(),
            Some(callback)=> callback(request),
        }
    }

}

fn main() {

    let mut router= FnPointerRouter::new();
    router.add_route("/", |_|get_form_response());
    
    router.handle_request(&Request { 
        method: String::from("post"), 
        url: "test".to_string(), 
        headers: HashMap::new(), 
        body: "{}".into()
    });
    
    router.handle_request(&Request { 
        method: String::from("post"), 
        url: "/".to_string(), 
        headers: HashMap::new(), 
        body: "{}".into()
    });

    println!("✅ Finalizado!");
}

fn get_form_response() -> Response{
    println!(" --> OK");
    Response{
        code: 200,
        headers: HashMap::new(),
        body: "ok".into()
    }
}


fn not_found_response() -> Response{
    println!(" --> No encontrado");
    Response{
        code: 404,
        headers: HashMap::new(),
        body: "No encontrado".into()
    }
}
