
trait Subprogramm {
    fn start(&self);
}

struct cd;

impl Subprogramm for cd{
    fn start(&self) {
        cd::runtime()
    }
}

impl cd {
    fn runtime() {

    }
}