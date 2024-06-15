use std::fmt::Display;
use std::ops::{Div, Sub};

pub struct Indicator<T>
{
    first: T,
    step: T,
    newest: usize,
    done: Vec<bool>,
}

impl<T> Indicator<T>
where
    T: PartialOrd + Clone + Sub<Output=T> + Div<Output=T> + Into<usize> + 'static,
{
    pub fn new_linear(mut iter: impl Iterator<Item=T>) -> Indicator<T> {
        let first = iter.next().unwrap();
        let next = iter.next().unwrap();
        let step = next.clone() - first.clone();
        let count = iter.count() + 2;

        // let transform = move |x: T| ((x - first.clone()) / step.clone()).into();
        // let transform = Box::new(transform);


        Indicator {
            first,
            step,
            newest: 0,
            done: vec![false; count],
        }
    }

    pub fn done(&mut self, x: T) {
        let index = ((x - self.first.clone()) / self.step.clone()).into();
        self.done[index] = true;
        self.newest = index;
    }
}

impl<T> Display for Indicator<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "│")?;
        let mut done_overall = 0;
        for (index, done) in self.done.iter().enumerate() {
            if *done {
                if index == self.newest {
                    write!(f, "\x1b[31m█\x1b[0m")?;
                } else {
                    write!(f, "█")?;
                }
                done_overall += 1;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, "│")?;
        write!(f, "  {:.2}%", done_overall as f64 / self.done.len() as f64 * 100.)?;
        Ok(())
    }
}

// fn get_transform<T>(mut iter: impl Iterator<Item=T>) -> fn(T) -> usize
// where
//     T: PartialOrd + Clone + Sub<Output=T> + Div<Output=T> + Into<usize> + 'static,
// {
//     let first = iter.next().unwrap();
//     let next = iter.nth(1).unwrap();
//     let step = next.clone() - first.clone();
//     let transform = move |x: T| ((x - first.clone()) / step.clone()).into();
//     transform
// }