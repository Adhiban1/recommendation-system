use rand::Rng;

fn main() {
    let movies = 100;
    let users = 10;
    let a = rating_matrix(users, movies);
    // println!("{:?}\n", a);

    let mut b = Vec::with_capacity(users - 1);
    for i in 1..users {
        b.push(sim(0, i, &a));
    }

    let index = sort(&mut b);

    // println!("{:?}\n\n{:?}", b, index);

    let r = rec(&a, &index);
    println!("User1's Ratings: \n{:?}\n\nRecommendation:\n{:?}", a[0], r);
}

fn rating_matrix(r:usize, c:usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut v = vec![vec![0.0;c];r];
    for i in 0..r {
        for j in 0..c {
            v[i][j] = rng.gen_range(0..=5) as f64;
        }
    }
    v
}

fn sim(a:usize, b:usize, m:&Vec<Vec<f64>>) -> f64 {
    let length = m[a].len();
    let a_mean = m[a].iter().sum::<f64>() / length as f64;
    let b_mean = m[b].iter().sum::<f64>() / length as f64;
    let mut mod_a = 0.0;
    let mut mod_b = 0.0;
    let mut s = 0.0;
    for j in 0..length {
        let mut a_temp = m[a][j];
        let mut b_temp = m[b][j];
        if a_temp != 0.0 {
            a_temp -= a_mean;
        }
        if b_temp != 0.0 {
            b_temp -= b_mean;
        }
        mod_a += a_temp.powi(2);
        mod_b += b_temp.powi(2);
        s += a_temp * b_temp;
    }
    s /= mod_a.sqrt() * mod_b.sqrt();
    s
}

fn sort(s:&mut Vec<f64>) -> Vec<usize> {
    let mut index:Vec<usize> = (0..s.len()).collect();
    for i in 0..s.len() {
        for j in 0..i {
            if s[i] > s[j] {
                let temp = s[i];
                s[i] = s[j];
                s[j] = temp;
                let temp = index[i];
                index[i] = index[j];
                index[j] = temp;
            }
        }
    }
    index
}

fn rec(rm:&Vec<Vec<f64>>, index:&Vec<usize>) -> Vec<usize> {
    let mut zeros = vec![];
    for i in 0..rm[0].len() {
        if rm[0][i] == 0.0 {
            zeros.push(i);
        }
    }

    let mut r = vec![];

    for i in index {
        for j in zeros.iter() {
            if rm[*i+1][*j] != 0.0 {
                let mut contain = false;
                for k in r.iter() {
                    if k == j {
                        contain = true;
                    }
                }
                if contain == false {
                    r.push(*j);
                }
            }
        }
    }
    r
}