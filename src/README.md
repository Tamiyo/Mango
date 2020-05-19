# Mango Technical Specs

## Variable Assignment

Mango currently supports the following assignable values (though more are to come):

*   Numerics
*   Booleans
*   Strings
*   Arrays
*   Closures
*   Classes

As of this point in time, Mango needs to implement native classes for arrays, dictionaries, etc.

Prmitives
<pre><code>myvar = 4; 
print(myvar); 
</code></pre>

<pre><code>>> 4</code></pre>

Arrays
<pre><code>myvar = [1, 2, 3]; 
print(myvar); 
</code></pre>
<pre><code>>> [1, 2, 3]</code></pre>

Functions
<pre><code>#f() {
    print("Hello World!");
}
myvar = f;
myvar();
</code></pre>
<pre><code>>> "Hello World!"</code></pre>

Classes
<pre><code>@MyClass {}
A = MyClass(); 
print(A); 
</code></pre>
<pre><code>>> instance of MyClass</code></pre>

## Conditionals
<pre><code>x = 4;
if(x < 6) {
    print("x < 6");
} elif (x > 7) {
    print("x > 7");
} else {
    print("x == 6");
}</code></pre>
<pre><code>>> "x < 6"</code></pre>

## Loops

While Loops
<pre><code>x = 0;
while(x < 5) {
    print(x);
    x = x + 1;
}</code></pre>

<pre><code>>> 0
1
2
3
4
5</code></pre>

For Loop #1
<pre><code>A = [2, 5, 6];
for a in A {
    print(a);
}</code></pre>

<pre><code>>> 2
5
6</code></pre>

For Loop #2
<pre><code>for a in 1:4 {
    print(a);
}</code></pre>

<pre><code>>> 1
2
3</code></pre>

## Classes
<pre><code>@MyClass {
    #init(x) {
        my.x = x;
    }
}
C = MyClass(10);
print(C.x);</code></pre>

<pre><code>10</code></pre>
