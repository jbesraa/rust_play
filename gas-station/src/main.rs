use std::collections::VecDeque;

// #[derive(Debug)]
// struct Vehicle {
//     kind: String,
//     id: i32,
//     service: String,
// }

// impl Vehicle {
//     fn new(kind: String, id: i32, service: String) -> Self {
//         Self { kind, id, service }
//     }
// }

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    let input: Vec<(i32, String, String)> = vec![
        (1, "private".to_string(), "C".to_string()),
        (2, "trailer".to_string(), "R".to_string()),
        (3, "private".to_string(), "R".to_string()),
        (4, "private".to_string(), "R".to_string()),
    ];
    let _time = 0;
    let mut count = 0;
    let mut cleaning_waiting_list: VecDeque<&(i32, String, String)> = VecDeque::new();
    let mut fueling_waiting_list: VecDeque<&(i32, String, String)> = VecDeque::new();
    let num_of_vehicles = input.len();
    let mut is_cleaning_available = true;
    let mut is_fueling_available = true;

    while count < num_of_vehicles
        || !cleaning_waiting_list.is_empty()
        || !fueling_waiting_list.is_empty()
    {
        let mut next_in_cleaning: Option<&(i32, String, String)>;
        let mut next_in_fueling: Option<&(i32, String, String)>;
        let next_vehicle: &(i32, String, String) = &input[count];

        if next_vehicle.2 == "C" && cleaning_waiting_list.is_empty() {
            next_in_cleaning = Some(next_vehicle);
        } else {
            cleaning_waiting_list.push_back(&next_vehicle);
        }
        if next_vehicle.2 == "R" && fueling_waiting_list.is_empty() {
            next_in_fueling = Some(next_vehicle);
        } else {
            fueling_waiting_list.push_back(&next_vehicle);
        }

        if !fueling_waiting_list.is_empty() {
            next_in_fueling = fueling_waiting_list.pop_front();
        }

        if !cleaning_waiting_list.is_empty() {
            next_in_cleaning = cleaning_waiting_list.pop_front();
        }
        count += 1;
    }
}
