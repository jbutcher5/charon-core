### Chapter 1

This is the first chapter of the W programming language.

## Container

A container is what is known as a function is many other programming languages. However, the core difference is that it's functions, prodeduce and variable. This means that it's extremely loose and generic to write. Building on that, this means that we can use a clear operator to show "storing" code in a container. This operator is `<-`. This means that we can write `x <- 5` to store 5 in the container x. Likewise, we can write `y <- 5 OUTPUT` to output 5 everytime we use the y container. Example code of this would be:

```
y <- 5 OUTPUT

y
```

On our first line we defining the y container with `5 OUTPUT` and later on our final line we are using our y container. Much like in other languages they also act as variables:

```
x <- 4
x OUTPUT

x <- 3
x OUTPUT
```

In this case the output would be:

```
4
3
```

This is because we have redefined the container x to 3 from 4. The content is just code, therefore we could redefine the contents container that performs an operation such as:

```
x <- 2 1 sub
x OUTPUT

x <- 2 1 add
x OUTPUT
```

Which would output:

```
1
3
```

### Parameters

Parameters are much like function parameters in any other language except Wlang conform to a postfix syntax style. The usage of parameters looks like this:

```
f <- $0 $1 sub $2 add

4 3 9 f OUTPUT
```

The `$` denotes a parameter and then is followed by an index. The index 0 corresponds to the value 9, index 1 corresponds to the value 3 and so on. The values are then copied in place of the parameters respectively and the container is executed. In this case the output would be 10 because 3 is subtracted from 9 which equates to 6 and then 6 is added to 4 which results in 10. The parameters that are used in the container are deleted so the whole section `4 3 9 f` would be replaced with 10 and is then outputed. However, if that parameter is never used in the container it would remain in place. This would allow for the following code to output `6 7`.

```
f <- $0 $2 add

3 6 4 f OUTPUT
```

This is becase the values 4 and 3 are deleted and then replaced (at the point where the function is called) with 7 which when outputed results in `6 7`.

### Ephemeral Containers

An ephemeral container is a container that does not modify the arguments it comes into contact with. You can not write an ephemeral container in W, you must write it in rust as to get around the evaluator deleteing parameters used by a container where you are able to add your own implmentation and have much more flexibilty in rust functions. An example of an ephemeral container is the OUTPUT container. When the container is used by the evaluator it does not interact with any of the parameters that come after it. For example `3 OUTPUT` implies `3` because the OUTPUT is deleted without a trace which therefore means that you can embed it into your code with little interference, this would look like: `2 3 OUTPUT sub OUTPUT` which would first output `2 3` and then the first output would delete it's self and then it would subtract 3 from 2 and result in -1 then output -1 through the final output.
