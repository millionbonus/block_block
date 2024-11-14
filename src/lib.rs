use std::fmt::Display;


pub struct Block<'a, TData> {
    pub name: Option<String>,
    pub exe_fn: Box<dyn Fn(&mut TData) + 'a>,
    pub next: Option<Box<Block<'a, TData>>>
}

impl<'a, TData> Block<'a, TData> {

    pub fn new() -> Block<'a, Option<TData>> {
        Block{
            name: None,
            exe_fn: Box::new(|_p| { }),
            next: None
        }
    }

    pub fn from(exe_fn:impl Fn(&mut TData) + 'a) -> Block<'a, TData> {
        Block{
            name: None,
            exe_fn: Box::new(exe_fn),
            next: None
        }
    }

    pub fn connect_with_fun(&mut self, exer_fn: impl Fn(&mut TData) + 'a) {
        self.connect(Block::from(exer_fn));
    }

    pub fn connect(&mut self, next:Block<'a, TData>) {
        self.next = Some(Box::new(next));
    }

    pub fn exec(&self, data: &mut TData) {

        let mut curr = self;
        curr.exe_fn.as_ref()(data);

        while let Some(n) = curr.next.as_ref() {
            n.exe_fn.as_ref()(data);
            curr = n;
        }
    }

    pub fn exec_by_iter(&self, data: &mut TData) {
        for b in self.iter() {
            (&b.exe_fn)(data);
        }
    }

    pub fn iter(&'a self) -> BlockInterator<'a, TData> {
        BlockInterator::<'a, TData> {
            init: self,
            curr: None
        }
    }
}

impl<'a, TData> Display for Block<'a, TData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "(name: {:?}, exe_fn: {:p}, next: {:p})",
               &self.name,
               &self.exe_fn,
               &self.next)
    }
}

pub struct BlockInterator<'a, TData> {
    init: &'a Block<'a, TData>,
    curr: Option<&'a Block<'a, TData>>
}

impl<'a, TData> Iterator for BlockInterator<'a, TData> {
    type Item = &'a Block<'a, TData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            self.curr = Some(self.init);
            return self.curr;
        }

        match self.curr {
            None => None,
            Some(x) => {
                match &x.next{
                    None => None,
                    Some(cc) => {
                        self.curr = Some(&*cc);
                        self.curr
                    }
                }
            }
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

    #[test]
    fn itor_test() {

        let mut b1 = Block::from(|p:&mut String| {
            p.push_str("_b1");
            dbg!(&p);
        });
        b1.name = Some(String::from("b1"));

        let mut b2 = Block::from(|p:&mut String| {
            p.push_str("_b2");
            dbg!(&p);
        });

        let b3 = Block::from(|p:&mut String| {
            p.push_str("_b3");
            dbg!(&p);
        });

        b2.connect(b3);
        b1.connect(b2);

        let mut data = String::from("haha");
        dbg!(&data);
        b1.exec_by_iter(&mut data);

        let expected = String::from("haha_b1_b2_b3");

        assert_eq!(expected, data);

    }
}
