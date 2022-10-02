trait Nat {
    fn nat() -> usize;
}

struct Zero;

impl Nat for Zero {
    fn nat() -> usize {
        0
    }
}

struct Next<N: Nat>(std::marker::PhantomData<N>);

impl<Prev: Nat> Nat for Next<Prev> {
    fn nat() -> usize {
        1 + Prev::nat()
    }
}

trait Add<Val: Nat> {
    type Sum;
}

impl<Val: Nat> Add<Val> for Zero {
    type Sum = Val;
}

impl<Prev: Nat, Val: Nat> Add<Val> for Next<Prev>
where
    Prev: Add<Val>,
    <Prev as Add<Val>>::Sum: Nat,
{
    type Sum = Next<<Prev as Add<Val>>::Sum>;
}

fn main() {
    println!(
        "3 + 2 = {}",
        <Next<Next<Next<Zero>>> as Add<Next<Next<Zero>>>>::Sum::nat()
    );
}
