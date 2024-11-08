use std::fmt::Display;


pub struct Block<'a, TData> {
    pub name: Option<String>,
    pub exe_fn: Box<dyn Fn(&mut TData) + 'a>,
    pub next: Box<Option<Block<'a, TData>>>
}

impl<'a, TData> Block<'a, TData> {

    pub fn new() -> Block<'a, Option<TData>> {
        Block{
            name: None,
            exe_fn: Box::new(|_p| { }),
            next: Box::new(None)
        }
    }

    pub fn from(exe_fn:impl Fn(&mut TData) + 'a) -> Block<'a, TData> {
        Block{
            name: None,
            exe_fn: Box::new(exe_fn),
            next: Box::new(None)
        }
    }

    pub fn connect_with_fun(&mut self, exer_fn: impl Fn(&mut TData) + 'a) {
        self.connect(Block::from(exer_fn));
    }

    pub fn connect(&mut self, next:Block<'a, TData>) {
        self.next = Box::new(Some(next));
    }

    pub fn exec(&self, data: &mut TData) {
        let mut curr = self;
        curr.exe_fn.as_ref()(data);

        while let Some(n) = curr.next.as_ref() {
            n.exe_fn.as_ref()(data);
            curr = n;
        }
    }
}

impl<'a, TData > Display for Block<'a, TData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "(name: {:?}, exe_fn: {:p}, next: {:p})",
               &self.name,
               &self.exe_fn,
               &self.next)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {


        let mut b1 = Block::from(|p:&mut String| {
            p.push_str("_b1");
            dbg!(&p);
        });
        b1.name = Some(String::from("b1"));

        let b2 = Block::from(|p:&mut String| {
            p.push_str("_b2");
            dbg!(&p);
        });

        println!("{}", b1.to_string());

        b1.connect(b2);

        println!("{}", b1.to_string());


        let mut data = String::from("haha");
        dbg!(&data);
        b1.exec(&mut data);

        let expected = String::from("haha_b1_b2");

        assert_eq!(expected, data);
    }
}
