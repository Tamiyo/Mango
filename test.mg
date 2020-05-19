@Array {
    #init(arr) {
        my.arr = arr;
    }

    #append(e) {
        print(e);
    }
}

A = Array([1,2,3]);
A.append(5);
print(A.arr);

#f() {
    x = 4;
    #g() {
        #k() {
            print(x);
        }
        k();
    }
    g();
}

f();

#fibo(n) {
    if (n == 0) {
        return 0;
    } elif (n == 1) {
        return 1;
    } else {
        return (fibo(n - 1) + fibo(n - 2));
    }
}

x = fibo(7);

print(x);