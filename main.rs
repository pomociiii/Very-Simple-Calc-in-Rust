use std::io;


fn multiply(x:i32, y:i32) -> i32
{
     x*y
}

fn subtract(x:i32, y:i32) -> i32
{
    x - y
}

fn divide(x:i32, y:i32) -> i32
{
    x / y
}

fn add(x:i32, y:i32) -> i32
{
    x+y
}

fn main()
{
    let mut x = String::new();
    let mut y = String::new();
    let mut q = String::new();
    
    loop{
        println!("Do you want to multiply something (1)
        Do you want to subtract something (2)
        Do you want to divide something (3)
        Do you want to add something (4)");

        q.clear();
        io::stdin()
        .read_line(&mut q)
        .expect("Failed to read line");
        println!("{}","what is your first number?");
        
        x.clear();
        io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
        
        println!("what is your second number?");

        y.clear();
        io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
        
        let x: i32 = match x.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        let y: i32 = match y.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
            
        };
        let q: i32 = match q.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        if q == 1
        {
            println!("{}",multiply(x,y));
        }
        else if q == 2
        {
            println!("{}",subtract(x,y));
        }
        else if q == 3
        {
            println!("{}",divide(x,y));
        }
        else if q == 4
        {
            println!("{}",add(x,y));
        }
        else 
        {
            println!("An error occurred!");
            continue;
        }
        
    }
    


}

