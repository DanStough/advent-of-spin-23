
use spin_sdk::{
    http::{IntoResponse, Request, Method, },
    http_component,
    key_value::Store,
};

// // Define a serializable User type
// #[derive(Serialize, Deserialize)]
// struct Payload {
//     value: String,
// }


/// A simple Spin HTTP component.
#[http_component]
fn handle_challenge(req: Request) -> anyhow::Result<impl IntoResponse> {

    println!("HERE");
       
    // Assume that the first query param is the data
    // let Some((key, val)) = params.next();
    let key = req.query();


      // Open the default key-value store
    let store = Store::open_default()?;

    println!("HERE");

    let (status, body) = match *req.method() {
        Method::Post => {
            // Add the request (URI, body) tuple to the store
            store.set(key, req.body())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                key
            );
            (201, None)
        }
        Method::Get => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(key)? {
                Some(value) => {
                    println!("Found value for the key {:?}", key);
                    (200, Some(value))
                }
                None => {
                    println!("No value found for the key {:?}", key);
                    (404, None)
                }
            }
        }
        // No other methods are currently supported
        _ => (405, None),
    };
    
    Ok(http::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(body)?)

    // println!("Handling request to {:?}", req.header("spin-full-url"));
    // Ok(http::Response::builder()
    //     .status(200)
    //     .header("content-type", "text/plain")
    //     .body("Hello, Fermyon")?)
}

