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
}