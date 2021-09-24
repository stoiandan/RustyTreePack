mod food;
mod node;

use food::Food;
use  rand::Rng; 
use self::node::Node;

fn main() {
    let foods = create_foods();

    println!("In order of calories to cost ratio:\n");
    for food in foods.iter() {
        print!("{}\n",food);
    }
}


fn create_tree(foods: &[Food])  {
   let root= Node::new(foods[0].cost);

    for food in &foods[1..] {
        
    }
}


fn create_foods<'a>() -> [Food<'a>; 10] {
   let mut rng = rand::thread_rng(); 
   let mut foods : [Food; 10] = [ Default::default(); 10];

   let food_names = ["Pizza", "Hamburger", "HotDog", "Soup", "Wine", "Toast", "Donut", "Nuts", "American Pancakes", "Wafels"];

   for  (idx, food) in foods.iter_mut().enumerate() {
       food.calories = rng.gen_range(100..=500);
       food.cost     = rng.gen_range(10..=100);
       food.name     = food_names[idx];
   }

   fn cost_calories_ratio(food: &Food) -> u32 {
       food.calories / food.cost
   }
   foods.sort_by_key(|el| cost_calories_ratio(el));

   foods.reverse();

   foods
}
