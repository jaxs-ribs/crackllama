use kinode_process_lib::{println, Request};
use crate::VECTORBASE_ADDRESS;

// TODO: Zena: Strip these tests out into a script inside of command centers vectorbase
pub fn temp_test() {
    {
        // List Database
        let request = vectorbase_interface::Request::ListDatabases;
        let response = Request::to(VECTORBASE_ADDRESS)
            .body(serde_json::to_vec(&request).unwrap())
            .send_and_await_response(30)
            .unwrap()
            .unwrap();
        if let vectorbase_interface::Response::ListDatabases(databases) =
            serde_json::from_slice(response.body()).unwrap()
        {
            println!("Databases are: {:?}", databases);
        } else {
            println!("ERROR: {:?}", response);
        }
    }
    {
        // Submit Data
        let request = vectorbase_interface::Request::SubmitData {
            database_name: "test4".to_string(),
            values: vec![
                ("id_001".to_string(), "Cats have retractable claws that help them climb and hunt.".to_string()),
                ("id_002".to_string(), "Dogs are known for their loyalty and are often called man's best friend.".to_string()),
                ("id_003".to_string(), "Cats can jump up to six times their length.".to_string()),
                ("id_004".to_string(), "Dogs have an excellent sense of smell and are used in search and rescue operations.".to_string()),
                ("id_005".to_string(), "Cats spend 70% of their lives sleeping.".to_string()),
                ("id_006".to_string(), "Dogs can understand up to 250 words and gestures.".to_string()),
                ("id_007".to_string(), "Cats have a third eyelid called the nictitating membrane.".to_string()),
                ("id_008".to_string(), "Dogs sweat through their paw pads.".to_string()),
                ("id_009".to_string(), "Cats have 32 muscles in each ear.".to_string()),
                ("id_010".to_string(), "Dogs have three eyelids, including one to keep their eyes moist and protected.".to_string()),
            ],
        };

        let response = Request::to(VECTORBASE_ADDRESS)
            .body(serde_json::to_vec(&request).unwrap())
            .send_and_await_response(30)
            .unwrap()
            .unwrap();
        if let vectorbase_interface::Response::SubmitData =
            serde_json::from_slice(response.body()).unwrap()
        {
            println!("Success populating!");
        } else {
            println!("error: {:?}", response);
        }
    }
    {
        // Semantic Search
        let request = vectorbase_interface::Request::SemanticSearch {
            database_name: "test4".to_string(),
            top_k: 3,
            query: "What are cats like?".to_string(),
        };

        let response = Request::to(VECTORBASE_ADDRESS)
            .body(serde_json::to_vec(&request).unwrap())
            .send_and_await_response(30)
            .unwrap()
            .unwrap();
        if let vectorbase_interface::Response::SemanticSearch(results) =
            serde_json::from_slice(response.body()).unwrap()
        {
            println!("Results are: {:?}", results);
        } else {
            println!("ERROR: {:?}", response);
        }
    }
}
