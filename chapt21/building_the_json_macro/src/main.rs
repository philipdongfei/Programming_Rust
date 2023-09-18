use std::collections::HashMap;
use std::boxed::Box;
use std::string::ToString;


#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}


impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };    
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
                        usize isize f32 f64);


#[recursion_limit = "256"]
macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![ $( json!($element) ),*  ])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        {
            // Scoping and Hygiene 
            let mut fields = Box::new(HashMap::new());
            $( fields.insert($key.to_string(), json!($value)); )*
            Json::Object(fields)

        }
        /*
        Json::Object(Box::new(vec![
                $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))
        */
    };
    ( $other:tt ) => {
        Json::from($other) // Handle Boolean/number/string
    };

}


fn main() {
    let width = 4.0;
    let desc = 
        json!({
            "width": width,
            "height": (width * 9.0 / 4.0)
        });
    println!("{:?}", &desc);


    let hand_coded_value = {
        let students = Json::Array(vec![
            Json::Object(Box::new(vec![
                    ("name".to_string(), Json::String("Jim Blandy".to_string())),
                    ("class_of".to_string(), Json::Number(1926.0)),
                    ("major".to_string(), Json::String("Tibetan throat singing".to_string()))
            ].into_iter().collect())),
            Json::Object(Box::new(vec![
                    ("name".to_string(), Json::String("Jason Orendorff".to_string())),
                    ("class_of".to_string(), Json::Number(1702.0)),
                    ("major".to_string(),  Json::String("Knots".to_string()))
            ].into_iter().collect()))
        ]);
        students
    };

    let macro_generated_value = {

        let students = json!([
            {
                "name": "Jim Blandy",
                "class_of": 1926,
                "major": "Tibetan throat singing"
            },
            {
                "name": "Jason Orendorff",
                "class_of": 1702,
                "major": "Knots"
            }
        ]);
        students
    };
    assert_eq!(macro_generated_value, hand_coded_value);

    // Scoping and Hygiene 
    let fields = "Fieldds, W.C.";
    let role = json!({
        "name": "Larson E. Whipsnade",
        "actor": fields
    }); 
    println!("{:?}", &role);
    
}



#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null); // passes!
}

#[test]
fn json_array_with_json_element() {
    
    let macro_generated_value = json!(
        [
            // valid JSON that doesn't match `$element:expr`
            {
                "pitch": 440.0
            } 
        ]
    );

    let hand_coded_value = 
        Json::Array(vec![
            Json::Object(Box::new(vec![
                    ("pitch".to_string(), Json::Number(440.0))
            ].into_iter().collect()))
        ]);
    assert_eq!(macro_generated_value, hand_coded_value);
}

