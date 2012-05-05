
enum e = int;

iface i { }

// crashes
impl of e for int { }

// works
// impl of i for int { }

fn main() { }

