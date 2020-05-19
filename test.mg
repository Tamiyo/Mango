start = clock();

#fibo(nth) {
    p_previous = 0;
    previous = 0;
    cur = 1;

    for i in 1:nth {
        p_previous = previous;
        previous = cur;
        cur = p_previous + previous;
    }

    return cur;
}


for i in 1:60 {
    print(fibo(i));
}

print("elapsed time (sec):", (clock() - start) / 1000);