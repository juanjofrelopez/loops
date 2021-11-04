fn main() {
    // este programa cuenta de 10 a 9, dos veces
    let mut count = 0;
    // labeled loop
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // breaks out of the inner loop
            }
            if count == 2 {
                break 'counting_up; // breaks out of the outter loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // loops can return a value after a condition was met
    let mut counter =  0;
    let result = loop {
        counter+=1;
        if counter == 10{
            break counter*2; // returns value from xpression
        }
    }; // hay que poner un punto y coma
    println!("result was: {}",result);

    // WHILE LOOP
    let mut number =3;
    while number != 0 {
        println!("{}",number);
        number-=1;
    }

    // FOR LOOP: iterating over elements of a collection
        // for example an array
    let a = [9,8,7,6,5,4];
    for every_element in a {
        println!("{}",every_element);
    }
        // short version of a countdown
        // using Range and rev()
    println!("COUNTDOWN");
    for number in (1..4).rev() {
        println!("{}",number);  
    }
}

/*
FOR loops increase the efficiency and safety. Also we eliminated
the chance of bugs because we are not managing index like
we'd do on C-style code.

When we want to run some code a certain number of times 
we use a for loop with a type called Range

*/