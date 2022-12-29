


use rand::{Rng};


fn meanSquaredLoss(yPred:Vec<f64>, yActual:Vec<f64>)->f64{
let mut loss = 0.0;
for (&p,&a) in yPred.iter().zip(yActual.iter()){
loss += (a-p)*(a-p);
}
loss = loss/(yActual.len() as f64);

loss

}

fn predict(x:Vec<f64>,m:f32,c:f32)->Vec<f64>{
    let y:Vec<f64> = x.iter().map(
        |v| -> f64 {v*m as f64 + c as f64}
    ).collect();
y
}


fn main()  {


    let mut rng = rand::thread_rng();
    let mut numbers: Vec<f64> = Vec::new();

    for _ in 0..100 {
        let number: f64 = rng.gen_range(0..128) as f64;
        numbers.push(number);
    }
    let m = 2.0;
    let c = 7.0;

    let y_actual:Vec<f64> = numbers.iter().map(
        |v| v*m + c
    ).collect();

    let m_trial = rng.gen_range(0..3) as f32;
    let c_trial = rng.gen_range(0..3) as f32;

    let y_pred = predict(numbers, m_trial, c_trial);
    
    // print!("{:?}",&numbers);



    


    
}
