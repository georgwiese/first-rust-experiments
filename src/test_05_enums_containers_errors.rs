struct TwoTupleStruct(i32, i32);
struct ThreeTupleStruct(i32, i32, i32);

enum Tuple {
    TwoTuple(TwoTupleStruct),
    ThreeTuple(ThreeTupleStruct),
}

impl Tuple {
    fn sum(&self) -> i32 {
        match self {
            Tuple::TwoTuple(tuple_struct) => tuple_struct.0 + tuple_struct.1,
            Tuple::ThreeTuple(tuple_struct) => tuple_struct.0 + tuple_struct.1 + tuple_struct.2,
        }
    }

    fn from_integer_vector(vec: Vec<i32>) -> Result<Tuple, String> {
        match vec.len() {
            2 => Ok(Tuple::TwoTuple(TwoTupleStruct(vec[0], vec[1]))),
            3 => Ok(Tuple::ThreeTuple(ThreeTupleStruct(vec[0], vec[1], vec[2]))),
            _ => Err(format!("Unsupported length: {}", vec.len())),
        }
    }
}

fn average_of_sum(vector: &Vec<Tuple>) -> Option<f32> {
    if vector.len() == 0 {
        None
    } else {
        let mut sum: f32 = 0.0;
        for tuple in vector {
            sum += tuple.sum() as f32;
        }
        Some(sum / (vector.len() as f32))
    }
}

fn main() {
    let tuple1 = Tuple::TwoTuple(TwoTupleStruct(2, 3));
    let tuple2 = Tuple::ThreeTuple(ThreeTupleStruct(4, 5, 6));
    let tuple_vec = vec![tuple1, tuple2];

    for (i, tuple) in tuple_vec.iter().enumerate() {
        println!("Sum of tuple {} is {}", i, tuple.sum());
    }
    println!(
        "The average of sum is {}",
        average_of_sum(&tuple_vec).unwrap()
    );

    let tuple_length_5 = Tuple::from_integer_vector(vec![1, 2, 3, 4, 5]);
    if let Err(error_message) = tuple_length_5 {
        println!("Error! {}", error_message);
    }

    if let None = average_of_sum(&Vec::<Tuple>::new()) {
        println!("Vectors without lengths have no average!");
    }
}
