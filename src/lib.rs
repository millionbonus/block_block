

// #[derive(Debug)]
pub struct Block<'a, T> {
    pub exe_fn: Box<dyn Fn(&mut T) + 'a>,
    pub next: Box<Option<Block<'a, T>>>
}

impl<'a, T> Block<'a, T> {

    pub fn new() -> Block<'a, Option<T>> {
        Block{
            exe_fn: Box::new(|p| { }),
            next: Box::new(None)
        }
    }

    pub fn from(exe_fn:impl Fn(&mut T) + 'a) -> Block<'a, T> {
        Block{
            exe_fn: Box::new(exe_fn),
            next: Box::new(None)
        }
    }

    pub fn connect_with_fun(&mut self, exer_fn: impl Fn(&mut T) + 'a) {
        self.connect(Block::from(exer_fn));
    }

    pub fn connect(&mut self, next:Block<'a, T>) {
        self.next = Box::new(Some(next));
    }

    pub fn exec(&self, data: &mut T) {
        let mut curr = self;
        curr.exe_fn.as_ref()(data);

        while let Some(n) = curr.next.as_ref() {
            n.exe_fn.as_ref()(data);
            curr = n;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {


        let mut b1 = Block::from(|p:&mut String| {
            p.push_str("_b1");
            println!("{}", p);
        });

        let mut b2 = Block::from(|p:&mut String| {
            p.push_str("_b2");
            println!("{}", p);
        });

        b1.connect(b2);

        let mut data = String::from("haha");
        println!("{}", data);
        b1.exec(&mut data);

        let expected = String::from("haha_b1_b2");

        assert_eq!(expected, data);
    }
}
