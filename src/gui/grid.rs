use std::cmp::min;
use nih_plug_vizia::vizia::prelude::*;

pub struct Grid {}

impl Grid {
    pub fn new<F>(cols: usize, modifiers: GridVerticalModifiers, cx: &mut Context, mut elements: Vec<F>) -> Handle<VStack>
        where
            F: FnOnce(&mut Context),
    {
        let len = elements.len();
        let rows = (len / cols) + (len % cols != 0) as usize;

        // TODO add padding
        VStack::new(cx, |cx| {
            for _ in 0..rows {
                let index = min(elements.len(), cols);
                let drain = elements.drain(0..index);
                HStack::new(cx, |cx| {
                    for element in drain {
                        element(cx);
                    }
                }).col_between(modifiers.col_between)
                    .child_top(modifiers.child_top)
                    .child_bottom(modifiers.child_bottom);
            }
        })
    }
}

pub struct GridVerticalModifiers {
    pub col_between: Units,
    pub child_top: Units,
    pub child_bottom: Units,
}


