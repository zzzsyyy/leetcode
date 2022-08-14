impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n<1 {return false}
        while n%2==0{n>>=1;}
        while n%3==0{n/=3;}
        while n%5==0{n/=5;}
        n==1
    }
}
