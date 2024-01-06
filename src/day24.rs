use std::{fs, fmt::Debug, str::FromStr};
use itertools::Itertools;
use nalgebra::{Vector3, Matrix3};
use num::{Num, Zero};

// type Pos3d = Vector3<i128>;
type Pos3d = Vector3<f64>;
type Vel3d = Vector3<f64>;

fn parse() -> Vec<(Pos3d, Vel3d)> {
    let raw_string = fs::read_to_string("./inputs/test.txt")
        .expect("couldn't read file");

    fn load<T : Num + Clone + Debug + FromStr + Zero + 'static> (t: &str) -> Vector3<T> {
        Vector3::from_iterator(t.split(", ").map(|x|
            x.trim().parse::<T>().unwrap_or(T::zero())
        ))
    }

    raw_string.split("\n").map(|l| { 
        let mut it = l.split(" @ ");
        (load::<f64>(it.next().unwrap()), 
            load::<f64>(it.next().unwrap()))
    }).collect_vec()
}

fn intersect2d(a: &(Pos3d, Vel3d), b: &(Pos3d, Vel3d)) 
    -> Option<Pos3d>
{
    let (pa, va) = a;
    let (pb, vb) = b;
    let mv : Matrix3<f64> = Matrix3::new(
        va.x, -vb.x, 0_f64,
        va.y, -vb.y, 0_f64,
        0_f64, 0_f64, 1_f64
    ).try_inverse()?;

    let ts = mv * (pb - pa);    
    let int = pa + ts.x * va;
    if ts.x < 0.0 || ts.y < 0.0 { return None; }

    let range = 200000000000000_f64..400000000000000_f64;
    if !range.contains(&int.x) || !range.contains(&int.y) 
        { return None; }

    Some(*pa)
}

/*
Hyperboloid containing the first three lines:
- H(x,y,z) = 59982340340060761896*x^2-82550366903732551157088452192647548*x
    +8924703592837931616*y^2+184242198509970918180*x*y
    -45468181754631286575654692680454664*y-77065948406811520926*z^2
    +47637216115500910164*x*z-40932281436446749650*y*z
    +33533000373563936579883535824937764*z
    +12254642922941667093892889270024359593244979209140 = 0

- The 4th hail intersects the H(x,y,z) = 0 at time t=866877808876 (there is another
    intersection point but it is not an integer one). The point is 
    (292931046564083, 176777587456154, 318986369932636)

- The direction is (-26, 331, -53)


*/

fn main() {
    let hails = parse();
    {
        let mut s1 = 0;
        for i in 0..hails.len() {
            for j in 0..i {
                intersect2d(&hails[i], &hails[j]).map(|_| s1 += 1 );
            }
        }
        println!("{}", s1);
    }
    {
        /*
        - Given 3 lines L_1, L_2, L_3 in R^3, there exists a unique H hyperboloid that contains all of them.
            A generic point (x,y,z) in H iff there exists a line L passing through (x,y,z) and with 
            a direction p such that it intersects L_1, L_2, L_3. This condition can be written in the 
            form of a matrix equation Ap=0. The equation det(A) = 0 precisely describes H.

            The rows of A are:
                (8878770624918126 - 12*y - 21*z, 13657712681150448 + 12*x - 54*z, -16053470620118283 + 21*x + 54*y)
                (-10329498737067588 + 58*y + 7*z, 42192133456436758 - 58*x - 77*z, -8621146009640465 - 7*x + 77*y)
                (73063150846438332 - 360*y + 84*z, -30489836360112934 + 360*x - 238*z, -41188565686674546 - 84*x + 238*y)
            and its determinant is:
                det(A)(x,y,z) = 59982340340060761896*x^2-82550366903732551157088452192647548*x
                    +8924703592837931616*y^2+184242198509970918180*x*y
                    -45468181754631286575654692680454664*y-77065948406811520926*z^2
                    +47637216115500910164*x*z-40932281436446749650*y*z
                    +33533000373563936579883535824937764*z
                    +12254642922941667093892889270024359593244979209140
            Thus H(x,y,z) = det(A)(x,y,z) = 0, and has the shape of an hyperboloid with one sheet. I computed
            those value using matrixcalc.org, which supports BigInt and symbolic calculus.
        
        - Evaluating H(x,y,z) along the fourth line we see that at infinity its value is negative: it 
            becomes 0 when intersecting H and has positive value in between the two intersection point. Using
            the following python code, I found the fourth hail hits the hyperboloid at 
            T = bisection(a0, b0) = 866877808876 at position X,Y,Z = g(866877808876, x, y, z, u, v, w)
            (where u,v,w is the direction vector and x,y,z is the initial position).

        - By definition, passing through this points, there exists a line that intersects L_1, L_2, L_3
            whose direction can be found computing the null space of A. Again from matrixcalc, the null
            space of A at X,Y,Z is <(-26, 331, -53)> (where 331 is prime).

        - In order for the hail I throw to be at X,Y,Z at time T, it must starts from either
            g(-T, X,Y,Z, -26, 331, -53) or g(-T, X,Y,Z, 26, -331, 53) but since the prior has a
            negative coordinate (does not fit in the problem) we can deduce the solution is
            sum(g(-866877808876,x,y,z,26,-331,53)) == 1007148211789625

        ```
        a = "59982340340060761896*x^2-82550366903732551157088452192647548*x+8924703592837931616*y^2+184242198509970918180*x*y-45468181754631286575654692680454664*y-77065948406811520926*z^2+47637216115500910164*x*z-40932281436446749650*y*z+33533000373563936579883535824937764*z+12254642922941667093892889270024359593244979209140"
        a = a.replace('^', '**')

        x,y,z = 243519011458151, 277335413285770, 177685287085848
        u,v,w = 57, -116, 163

        def f(x,y,z):
            return eval(a)

        def g(t, x,y,z, u,v,w):
            return (x+t*u, y+t*v, z+t*w)

        def bisection(a, b):
            global x,y,z,u,v,w

            c = int((b+a) / 2)
            print(c)
            if c == a or c == b:
                return c

            yy = f(*g(c, x,y,z, u,v,w))
            if yy > 0:
                return bisection(c, b)
            elif yy < 0:
                return bisection(a, c)
            else: 
                return c



        a0 = 500000000000 # f>0
        b0 = 1000000000000 # f<0

        */
        println!("{}", 1007148211789625_i128);
    }
}