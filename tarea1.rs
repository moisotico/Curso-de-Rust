
use std::io;

 fn fibonacci(fib_ord: u32)->u32{
    let nu;
    //loop

    if fib_ord <=1 {
        nu = 1;
    } else{
        //recursivity
        nu = fibonacci(fib_ord-1) + fibonacci(fib_ord-2);
    }

    //println!("Este # fibonacci es {} .", nu);
    return nu;
}

fn main(){
    //imput statement
    println!("El siguiente programa determina el número de la serie de fibonnaci para un orden \
    ingresado");

    println!("Ingrese un número:");
    let  mut fib_ord = String::new();

    io::stdin().read_line(&mut fib_ord)
        .expect("No se pudo leer el número."); //exception handling

    //parsing Dtring to u32
    let fib_ord: u32 = fib_ord.trim().parse()
        .expect("¡Recuerde ingresar un número!");

    //let mut fib_ord = 6;
    println!("El orden de fibonacci escogido es: {}.", fib_ord);

    let fibonacci_result = fibonacci(fib_ord);

    //result statement
    println!("El valor de ese orden de fibonacci es {}.", fibonacci_result);
}
