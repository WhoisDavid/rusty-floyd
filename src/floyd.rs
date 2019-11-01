use std::cmp;
use std::fmt;

struct Racer {
    pub pos: usize,
    pub val: u64,
}

impl fmt::Debug for Racer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Racer {{ pos: {}, val: {} }}", self.pos, self.val)
    }
}

impl PartialEq for Racer {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

pub fn floyd_cycle_entry_detection(vector: &Vec<u64>) -> (u64, usize, usize) {
    // Root is at the end of the vector
    let root = vector.len() - 1;
    let tortoise = &mut Racer {
        pos: root,
        val: vector[root],
    };
    let hare = &mut Racer {
        pos: root,
        val: vector[root],
    };

    vec_jump(&vector, tortoise, 1);
    vec_jump(&vector, hare, 2);

    while tortoise != hare {
        vec_jump(&vector, tortoise, 1);
        vec_jump(&vector, hare, 2);
    } // Tortoise and hare meet in the cycle

    // Tortoise and hare meet!
    // We now detected a cycle
    // This meeting point and the root are both at the same distance to the cycle entry point
    // Therefore we reset the tortoise and slow down the hare
    // such that they meet again at the cycle entry point
    tortoise.pos = root;
    tortoise.val = vector[root];

    while tortoise != hare {
        // dbg!(&tortoise, &hare);
        vec_jump(&vector, tortoise, 1);
        vec_jump(&vector, hare, 1);
    } // Tortoise and hare meet at the entry point

    return (
        tortoise.val,
        cmp::min(tortoise.pos, hare.pos),
        cmp::max(tortoise.pos, hare.pos),
    );
}

fn vec_jump(vector: &Vec<u64>, racer: &mut Racer, jumps: u8) {
    for _ in 0..jumps {
        racer.pos = racer.val as usize;
        racer.val = vector[racer.pos];
    }
}
