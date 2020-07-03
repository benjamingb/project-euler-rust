// Implementing the trait `std::fmt::Debug`
// to allow a struct instance to be printed
#[derive(Debug)]
pub struct Item {
    pub id: u32,
    title: String,
    problem: String,
}

/*
impl Item {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn problem(&self) -> &str {
        self.problem.as_str()
    }
}
*/

pub fn list() -> Vec<Item> {
   let v = vec![
        Item {
            id: 1,
            title: "Multiples of 3 and 5".to_string(),
            problem: "
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000
        ".to_string(),
        }, 
        Item {
            id: 2,
            title: "xMultiples of 3 and 5".to_string(),
            problem: "
xIf we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000
        ".to_string(),
        }
    ];

    //let x = a.get(0).;

    //println!("{}",1);

    
    
   // println!("The third element is {}", 1);


    v
}

