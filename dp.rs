fn main() {
    let w = [3,5,5,2].to_vec();
    let v = [7,9,9,2].to_vec();
    let W = 9;

    let mut vec = vec![0;W+1];
    for i in 0..v.len() {
        for j in (w[i]..=W).rev() {
            vec[j] = vec[j].max(vec[j-w[i]]+v[i]);
        }
    }
    println!("{:?}", vec[W]);
}

// input
// w<-[3,5,5,2] entries tableau
// v<-[7,9,9,2] entries tableau
// W<-9 entries

// var
// vec<-[0;W+1] entries tableau

// début
//     pour i<-0 à v.len()
//         pour j<-W à =w[i]
//             vec[j] <- max(vec[j], vec[j-w[i]]+v[i])
//         fin pour
//     fin pour
//     renvoyer vec[W]
// fin

