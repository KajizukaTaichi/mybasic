let number = 5
call factorial
exit

sub factorial
    let count = 0
    let result = 1
    loop_start: let count = count + 1
        let result = result * count
        if count < number then goto loop_start
    return
