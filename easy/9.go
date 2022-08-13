func isPalindrome(x int) bool {
    if x<0{
        return false
    }
    num := x
    y:=0
    for true{
        y = y*10 + x%10
        x/=10
        if x<=0{
            break
        }
    }
    return num==y
}
