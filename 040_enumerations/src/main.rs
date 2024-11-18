#![allow(dead_code)]

#[derive(Debug)]
enum CardValues {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

struct Guest {
    name: String,
}

struct Maintenance {
    phone_number_of_plumber: String
}

enum HotelRoomStatus {
    Occupied(Guest),
    Vacant,
    Maintenance(Maintenance),
    Closed
}

fn main() {
    let card = CardValues::Nine;
    println!("{:?}", card);

    // Print card as integer
    println!("{}", card as i32);

    let maintenance_room = HotelRoomStatus::Maintenance(Maintenance { phone_number_of_plumber: "123-456-7890".to_string() });
    let g = Guest { name: "John".to_string() };
    //let guest_room = HotelRoomStatus::Occupied(g);
    let mut my_room = HotelRoomStatus::Occupied(g);

    match &my_room {
        HotelRoomStatus::Occupied(guest) => println!("Occupied by {}", guest.name),
        HotelRoomStatus::Vacant => println!("Vacant"),
        HotelRoomStatus::Maintenance(maintenance) => println!("Maintenance: {}", maintenance.phone_number_of_plumber),
        HotelRoomStatus::Closed => println!("Closed"),
    }

    //println!("{:?}", g.name);
    
    if let HotelRoomStatus::Occupied(guest) = &mut my_room {
        println!("Occupied by {}", guest.name);
        guest.name.push('!');
    }
}
