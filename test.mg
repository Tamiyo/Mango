start = clock();

@MyClass {
    #init() {
        my.name = "myclass";
    }
}

#fibo(n) {
    if (n == 0) {
        return 0;
    } elif (n == 1) {
        return 1;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}

A = [2, 4, 6];
for x in A {
    print('A', x);
}

for x in 1:28 {
    print(fibo(x));
}

C = MyClass();
print(C.name);

print("elapsed time (sec):", (clock() - start) / 1000);