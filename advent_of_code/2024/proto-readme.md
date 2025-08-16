# The Carousel of Traits

- `Debug`: almost always want this
- `Clone`: can I make a copy of me?
- `Copy`: is making a copy of me never a mistake?
- `PartialEq`: can you ask if I'm equal to another of me?
- `Eq`: is there always some other me that would be equal to me?
- `PartialOrd`: can you ask if I'm greater / lesser than another of me?
- `Ord`: is there always an answer to that question?
- `Hash`: do I make sense as the key of a hash table?

# Coordinates. How do they work?

```
  0123
0 ABC
1 DEF
2 GHI
3
```

```
012
345
678
```

i = x + y * w

# Fn traits

```rust
fn foo(callback: fn() -> bool) {
    while !callback() {
        println!("Callback returned false, continuing to loop.");
    }
    println!("Callback returned true, stopping the loop!");
}

fn some_weird_function() -> bool {
    return true;
}

fn main() {
    foo(some_weird_function); // this would work
    foo(|| {
        return true;
    }); // this also would work
    let mut count = 10;
    foo(|| { // this is called a closure
        if count == 0 {
            return true;
        } else {
            count -= 1;
            return false;
        }
    }); // this would NOT WORK!!!! :O
}

// what is actually happening:
struct MyClosure {
    count: &mut i32,
}

// (not syntactically valid, but if it were...)
impl FnMut()->bool for MyClosure {
    fn call(&mut self) -> bool {
        if *self.count == 0 {
            return true;
        } else {
            *self.count -= 1;
            return false;
        }
    }
}
```

So we've got three "functionishness" traits:

- FnMut: I can call the function, its self is `&mut self` (so changes are allowed)
- Fn: I can call the function, its self is `&self` (so no changes allowed)
- FnOnce: I can call the function ONLY ONCE, its self is `self` (so it is consumed and it doesn't matter if it changes things)

When you're taking in one of these, you want to use the least restrictive (most restricted) one that fits. A more restricted one can be used in the place of a less restricted one. "WHAT THE FUCK IS THE ENGLISH LANGUAGE! RAAAAAAAAAAAAR!" -dvd 2025, speaking some serious truth bombs

# why is post-/pre- -increment/-decrement bullshit

```javascript
let a = 5;
let b = a++;

function frell(x, y) {
    return x * y;
}

console.log(frell(++b, b++)); // what will be printed?
```

and that's why pre-/post- -increment/-decrement is bullshit.

# time to become the daedric prince of linear algebra

```
Vector
Vector+Vector = Vector
Vector-Vector = Vector
Vector*i32 = Vector
Vector/i32 = Vector
-Vector = Vector

Point
Point+Vector = Point
Point-Point = Vector
```

We actually WERE in three dimensions! Here is the true form of these types!

```rust
struct Vector {
    x: i32,
    y: i32,
    // if you had a z it would go here
    w: 0, // w = "pointiness", 0 = not pointy
}

struct Point {
    x: i32,
    y: i32,
    // if you had a z it would go here
    w: 1, // 1 = yes pointy
}
```

We can't add two points because pointiness would be 2. We can't do arbitrary multiplications/divisions because pointiness would be weird other values.

But we could do this...

```rust
out_point = (san_francisco + salt_lake_city) / 2;
```

You can do any sequence of operations as long as it *ends* with `w` being equal to zero (Vector) or one (Point).

# Linear algebra!

You know what a vector is.

x, y, z coordinates, each pointing in an orthogonal direction.

```rust
// Translate a point along an offset
fn translate(original: Point, offset: Vector) -> Point {
    Point {
        x: original.x + offset.x,
        y: original.y + offset.y,
        z: original.z + offset.z,
        //w: original.w + offset.w, // this is okay because offset.w would be zero
    }
}

// Scale a point relative to the origin
fn scale(original: Point, factor: Vector) -> Point {
    Point {
        x: original.x * factor.x,
        y: original.y * factor.y,
        z: original.z * factor.z,
        //w: original.w,
    }
}

// Rotate a point around the origin, around the Z axis
fn rotate_z(original: Point, angle: f32) -> Point {
    let s = angle.sin();
    let c = angle.cos();
    /*
if y is 0:
x = original.x * c
y = original.y * s

if x is 0:
x = -original.y * s
y = original.x * c

we can always add components of a vector together to get the final vector, so
just add them
    */
    Point {
        x: original.x * c - original.y * s,
        y: original.x * s + original.y * c,
        z: original.z, // uh oh?
        //w: original.w,
    }
}

fn rotate_x(original: Point, angle: f32) -> Point {
    Point {
        x: original.x,
        y: original.y * c - original.z * s,
        z: original.y * s + original.z * c,
    }
}

fn rotate_y(original: Point, angle: f32) -> Point {
    Point {
        x: original.x * c - original.z * s,
        y: original.y,
        z: original.x * s + original.z * c,
    }
}
```

```

    / < the new point
 H /|
  / | Opposite (to our angle) = new Y coordinate
 /  |
/___|<-always 90 degrees
^ Adjacent (to our angle) = new X coordinate
the origin

S(in)=(O)pposite/(H)ypotenuse
C(os)=(A)djacent/(H)ypotenuse
T(an)=(O)pposite/(A)djacent

What if H is 1?

S(in)=(O)pposite
C(os)=(A)djacent
T(an)=(O)pposite/(A)djacent
```

```
x' = x*xx + y*xy + z*xz + w*xw
y' = x*yx + y*yy + z*yz + w*yw
z' = x*zx + y*zy + z*zz + w*zw
w' = x*wx + y*wy + z*wz + w*ww
```

```
translation:
1 0 0 tx       | x' = [1 *] x + [w *] tx
0 1 0 ty       | y' = [1 *] y + [w *] ty
0 0 1 tz       | z' = [1 *] z + [w *] tz
0 0 0 1        | w' =           [w *] 1

scaling:
sx 0 0 0       | x' = x * sx [+ y * 0 + z * 0 + w * 0]
0 sy 0 0       | y' = y * sy
0 0 sz 0       | z' = z * sz
 0 0 0 1       | w' = w * 1

rotate around z:
 c s 0 0
-s c 0 0
 0 0 1 0
 0 0 0 1
```

a * b * c = a * (b * c)

this way you can do any number of transformations with the same work

instead of having to do this for every vertex:
```rust
rotate_z(scale(translate(old_vertex, ...), ...), ...)
```

we do it for one set of transformation sets, and then just multiply every vector by the matrix

## the other 20%

Q: ~~What~~ [How] is a dot product?

```rust
fn dot_product(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
```

Q: What is a dot product **for**?

Dot product is for finding parallelness. If two vectors are parallel, their dot product will be big. If they're perpendicular, it'll be small. If they're antiparallel, it'll be negative.

Q: What is the dot product of two unit vectors **for**?

1 = perfectly parallel, 0 = perfectly perpendicular, -1 = perfectly antiparallel (opposite)

Q: What about NON unit vectors?

The above, but you multiply the magnitudes too. So if the magnitudes are A and B:

A\*B = perfectly parallel, 0 = perfectly perpendicular, -(A\*B) = perfectly antiparallel (opposite)

Q: That doesn't sound very useful

Yeah, normally, in graphics and games, you only want to dot product unit vectors together, and if one or the other isn't a unit it was an accident.

Q: What is a cross product **for**?

Cross product is for finding one vector that's perpendicular with two others.

With unit vectors, it's equivalent to holding out a hand, pointing your thumb along one vector, pointing your pointer finger along another vector, and sticking your middle finger out. Your middle finger is perpendicular to both thumb and pointer finger, and is the result of the cross product. (Whether this is a left-handed cross product or a right-handed cross product depends on which hand you use.)

Q: How is a cross product?

I don't remember. Use your local vector library. :)

Q: What if they're not unit vectors?

Then the result is multiplied by *twice the area of the triangle*. (???) Or, another way of thinking of it, is it's multiplied by the area of the square:

- point 1: [0,0,0]
- point 2: A
- point 3: A+B
- point 4: B

```
    A______
   /      /
  /      /
 /      /
/______B
```

Q: That doesn't sound very useful

Yeah, normally it's just like with dot products, you want to use unit vectors. There are a few tiny cases where it's not like that but you will never run into them.[citation needed]

## the other 1%

inverting a transformation matrix according to math rules (i.e. using your local vector library instead of remembering how) gives you the reverse of the transformation. if the old matrix rotated by 30°, its inverse rotates by -30°. etc.
